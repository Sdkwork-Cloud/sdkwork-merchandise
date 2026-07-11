use sdkwork_contract_service::CommerceServiceError;
use sdkwork_database_id::IdGenerator;
use sdkwork_merchandise_service::{
    CreateSingleSkuMerchandiseCommand, SingleSkuMerchandiseListQuery, SingleSkuMerchandisePage,
    SkuRecord, UpdateSingleSkuMerchandiseCommand,
};
use sqlx::{PgPool, Postgres, Transaction};

use super::shared::{
    ensure_fulfillment_type, ensure_single_sku_count, ensure_sku_spu_link, finalize_page,
    next_entity_id, store_error, updated_spec_json, verify_create_replay, verify_spu_replay,
    SpuReplayRecord,
};

pub(super) async fn list(
    pool: &PgPool,
    query: SingleSkuMerchandiseListQuery,
) -> Result<SingleSkuMerchandisePage, CommerceServiceError> {
    let rows = sqlx::query(
        r#"
        SELECT sku.id,
               sku.tenant_id,
               sku.organization_id,
               sku.spu_id,
               sku.sku_no,
               COALESCE(NULLIF(sku.name, ''), sku.title, sku.id) AS name,
               COALESCE(NULLIF(sku.title, ''), sku.name, sku.id) AS title,
               CAST(COALESCE(sku.price_amount, '0') AS TEXT) AS price_amount,
               CAST(sku.original_price_amount AS TEXT) AS original_price_amount,
               COALESCE(sku.currency_code, 'CNY') AS currency_code,
               COALESCE(sku.fulfillment_type, '') AS fulfillment_type,
               COALESCE(sku.inventory_tracking, 'untracked') AS inventory_tracking,
               COALESCE(sku.status, 'draft') AS status,
               NULL::text AS published_at,
               COALESCE(sku.spec_json, '{}') AS spec_json,
               CAST(sku.created_at AS TEXT) AS created_at,
               CAST(sku.updated_at AS TEXT) AS updated_at
        FROM commerce_product_sku sku
        LEFT JOIN commerce_product_spu spu
          ON spu.tenant_id = sku.tenant_id
         AND spu.organization_id = sku.organization_id
         AND spu.id = sku.spu_id
        WHERE sku.tenant_id = CAST($1 AS TEXT)
          AND ($2::text IS NULL OR sku.organization_id = CAST($2 AS TEXT))
          AND LOWER(COALESCE(sku.fulfillment_type, '')) = LOWER($3)
          AND ($4::text IS NULL OR LOWER(COALESCE(sku.status, '')) = LOWER($4))
          AND (
                $5::text IS NULL
                OR LOWER(COALESCE(NULLIF(sku.title, ''), sku.name, sku.id)) LIKE '%' || LOWER($5) || '%'
                OR LOWER(COALESCE(NULLIF(spu.title, ''), spu.name, spu.id)) LIKE '%' || LOWER($5) || '%'
                OR LOWER(COALESCE(sku.sku_no, '')) LIKE '%' || LOWER($5) || '%'
          )
        ORDER BY sku.updated_at DESC, sku.id DESC
        LIMIT $6 OFFSET $7
        "#,
    )
    .bind(&query.tenant_id)
    .bind(query.organization_id.as_deref())
    .bind(&query.fulfillment_type)
    .bind(query.status.as_deref())
    .bind(query.search_term.as_deref())
    .bind(query.page_size.saturating_add(1))
    .bind(query.offset)
    .fetch_all(pool)
    .await
    .map_err(|error| store_error("failed to list single-SKU merchandise", error))?;

    let mut items = rows
        .iter()
        .map(crate::postgres_catalog::map_sku_row)
        .collect::<Vec<_>>();
    Ok(finalize_page(&query, &mut items))
}

pub(super) async fn create(
    pool: &PgPool,
    id_generator: &dyn IdGenerator,
    command: CreateSingleSkuMerchandiseCommand,
) -> Result<SkuRecord, CommerceServiceError> {
    let identity = command.stable_identity();
    let persisted_spec = command.persisted_spec()?;
    let spec_json = serde_json::to_string(&persisted_spec)
        .map_err(|error| CommerceServiceError::validation(format!("serialize spec: {error}")))?;
    let mut tx = pool
        .begin()
        .await
        .map_err(|error| store_error("failed to begin single-SKU create transaction", error))?;

    if let Some(record) = load_sku_by_no(
        &mut tx,
        &command.tenant_id,
        &command.organization_id,
        &identity.sku_no,
    )
    .await?
    {
        verify_create_replay(&record, &command, &identity.sku_no, &persisted_spec)?;
        let spu = load_spu_by_no(
            &mut tx,
            &command.tenant_id,
            &command.organization_id,
            &identity.spu_no,
        )
        .await?
        .ok_or_else(|| CommerceServiceError::conflict("merchandise SPU for SKU was not found"))?;
        verify_spu_replay(&spu, &command, &identity.spu_no)?;
        ensure_sku_spu_link(&record, &spu)?;
        ensure_single_sku_spu(
            &mut tx,
            &command.tenant_id,
            &command.organization_id,
            &record.spu_id,
        )
        .await?;
        tx.commit().await.map_err(|error| {
            store_error("failed to commit single-SKU replay transaction", error)
        })?;
        return Ok(record);
    }

    let generated_spu_id = next_entity_id(id_generator, "merchandise SPU")?;

    sqlx::query(
        r#"
        INSERT INTO commerce_product_spu
            (id, tenant_id, organization_id, spu_no, name, title, status, created_at, updated_at)
        VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), $4, $5, $5, $6,
                CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(&generated_spu_id)
    .bind(&command.tenant_id)
    .bind(&command.organization_id)
    .bind(&identity.spu_no)
    .bind(&command.title)
    .bind(&command.status)
    .execute(&mut *tx)
    .await
    .map_err(|error| store_error("failed to create merchandise SPU", error))?;

    let spu = load_spu_by_no(
        &mut tx,
        &command.tenant_id,
        &command.organization_id,
        &identity.spu_no,
    )
    .await?
    .ok_or_else(|| CommerceServiceError::storage("created merchandise SPU was not found"))?;
    verify_spu_replay(&spu, &command, &identity.spu_no)?;
    let generated_sku_id = next_entity_id(id_generator, "merchandise SKU")?;

    sqlx::query(
        r#"
        INSERT INTO commerce_product_sku
            (id, tenant_id, organization_id, spu_id, sku_no, name, title, price_amount,
             original_price_amount, currency_code, fulfillment_type, inventory_tracking,
             status, spec_json, created_at, updated_at)
        VALUES (CAST($1 AS TEXT), CAST($2 AS TEXT), CAST($3 AS TEXT), CAST($4 AS TEXT), $5, $6, $6,
                $7, $8, $9, $10, 'untracked', $11, $12, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(&generated_sku_id)
    .bind(&command.tenant_id)
    .bind(&command.organization_id)
    .bind(&spu.id)
    .bind(&identity.sku_no)
    .bind(&command.title)
    .bind(command.price_amount.as_str())
    .bind(
        command
            .original_price_amount
            .as_ref()
            .map(|value| value.as_str()),
    )
    .bind(&command.currency_code)
    .bind(&command.fulfillment_type)
    .bind(&command.status)
    .bind(&spec_json)
    .execute(&mut *tx)
    .await
    .map_err(|error| store_error("failed to create merchandise SKU", error))?;

    let record = load_sku_by_no(
        &mut tx,
        &command.tenant_id,
        &command.organization_id,
        &identity.sku_no,
    )
    .await?
    .ok_or_else(|| CommerceServiceError::storage("created merchandise SKU was not found"))?;
    verify_create_replay(&record, &command, &identity.sku_no, &persisted_spec)?;
    ensure_sku_spu_link(&record, &spu)?;
    ensure_single_sku_spu(
        &mut tx,
        &command.tenant_id,
        &command.organization_id,
        &record.spu_id,
    )
    .await?;
    tx.commit()
        .await
        .map_err(|error| store_error("failed to commit single-SKU create transaction", error))?;
    Ok(record)
}

pub(super) async fn update(
    pool: &PgPool,
    command: UpdateSingleSkuMerchandiseCommand,
) -> Result<SkuRecord, CommerceServiceError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|error| store_error("failed to begin single-SKU update transaction", error))?;
    let current = load_sku(
        &mut tx,
        &command.tenant_id,
        &command.organization_id,
        &command.sku_id,
    )
    .await?
    .ok_or_else(|| CommerceServiceError::not_found("merchandise SKU was not found"))?;
    ensure_fulfillment_type(&current, &command.fulfillment_type)?;
    ensure_single_sku_spu(
        &mut tx,
        &command.tenant_id,
        &command.organization_id,
        &current.spu_id,
    )
    .await?;
    let spec_json = updated_spec_json(&current, &command)?;

    let sku_result = sqlx::query(
        r#"
        UPDATE commerce_product_sku
        SET name = COALESCE($1, name),
            title = COALESCE($1, title),
            price_amount = COALESCE($2, price_amount),
            original_price_amount = CASE
                WHEN $3 THEN $4
                ELSE original_price_amount
            END,
            currency_code = COALESCE($5, currency_code),
            status = COALESCE($6, status),
            spec_json = $7,
            updated_at = CURRENT_TIMESTAMP
        WHERE tenant_id = CAST($8 AS TEXT) AND organization_id = CAST($9 AS TEXT)
          AND id = CAST($10 AS TEXT)
          AND LOWER(COALESCE(fulfillment_type, '')) = LOWER($11)
        "#,
    )
    .bind(command.title.as_deref())
    .bind(command.price_amount.as_ref().map(|value| value.as_str()))
    .bind(command.original_price_amount.is_some())
    .bind(
        command
            .original_price_amount
            .as_ref()
            .and_then(|value| value.as_ref())
            .map(|value| value.as_str()),
    )
    .bind(command.currency_code.as_deref())
    .bind(command.status.as_deref())
    .bind(&spec_json)
    .bind(&command.tenant_id)
    .bind(&command.organization_id)
    .bind(&command.sku_id)
    .bind(&command.fulfillment_type)
    .execute(&mut *tx)
    .await
    .map_err(|error| store_error("failed to update merchandise SKU", error))?;
    if sku_result.rows_affected() != 1 {
        return Err(CommerceServiceError::not_found(
            "merchandise SKU was not found",
        ));
    }

    update_spu(&mut tx, &current, &command).await?;
    let updated = load_sku(
        &mut tx,
        &command.tenant_id,
        &command.organization_id,
        &command.sku_id,
    )
    .await?
    .ok_or_else(|| CommerceServiceError::storage("updated merchandise SKU was not found"))?;
    tx.commit()
        .await
        .map_err(|error| store_error("failed to commit single-SKU update transaction", error))?;
    Ok(updated)
}

async fn update_spu(
    tx: &mut Transaction<'_, Postgres>,
    current: &SkuRecord,
    command: &UpdateSingleSkuMerchandiseCommand,
) -> Result<(), CommerceServiceError> {
    let result = sqlx::query(
        r#"
        UPDATE commerce_product_spu
        SET name = COALESCE($1, name),
            title = COALESCE($1, title),
            status = COALESCE($2, status),
            updated_at = CURRENT_TIMESTAMP
        WHERE tenant_id = CAST($3 AS TEXT) AND organization_id = CAST($4 AS TEXT)
          AND id = CAST($5 AS TEXT)
        "#,
    )
    .bind(command.title.as_deref())
    .bind(command.status.as_deref())
    .bind(&command.tenant_id)
    .bind(&command.organization_id)
    .bind(&current.spu_id)
    .execute(&mut **tx)
    .await
    .map_err(|error| store_error("failed to update merchandise SPU", error))?;
    if result.rows_affected() != 1 {
        return Err(CommerceServiceError::conflict(
            "merchandise SPU for SKU was not found",
        ));
    }
    Ok(())
}

async fn load_sku(
    tx: &mut Transaction<'_, Postgres>,
    tenant_id: &str,
    organization_id: &str,
    sku_id: &str,
) -> Result<Option<SkuRecord>, CommerceServiceError> {
    let row = sqlx::query(
        r#"
        SELECT id, tenant_id, organization_id, spu_id, sku_no,
               COALESCE(NULLIF(name, ''), title, id) AS name,
               COALESCE(NULLIF(title, ''), name, id) AS title,
               CAST(COALESCE(price_amount, '0') AS TEXT) AS price_amount,
               CAST(original_price_amount AS TEXT) AS original_price_amount,
               COALESCE(currency_code, 'CNY') AS currency_code,
               COALESCE(fulfillment_type, '') AS fulfillment_type,
               COALESCE(inventory_tracking, 'untracked') AS inventory_tracking,
               COALESCE(status, 'draft') AS status,
               NULL::text AS published_at,
               COALESCE(spec_json, '{}') AS spec_json,
               CAST(created_at AS TEXT) AS created_at,
               CAST(updated_at AS TEXT) AS updated_at
        FROM commerce_product_sku
        WHERE tenant_id = CAST($1 AS TEXT) AND organization_id = CAST($2 AS TEXT)
          AND id = CAST($3 AS TEXT)
        LIMIT 1
        FOR UPDATE
        "#,
    )
    .bind(tenant_id)
    .bind(organization_id)
    .bind(sku_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(|error| store_error("failed to load merchandise SKU", error))?;
    Ok(row.as_ref().map(crate::postgres_catalog::map_sku_row))
}

async fn load_sku_by_no(
    tx: &mut Transaction<'_, Postgres>,
    tenant_id: &str,
    organization_id: &str,
    sku_no: &str,
) -> Result<Option<SkuRecord>, CommerceServiceError> {
    let row = sqlx::query(
        r#"
        SELECT id, tenant_id, organization_id, spu_id, sku_no,
               COALESCE(NULLIF(name, ''), title, id) AS name,
               COALESCE(NULLIF(title, ''), name, id) AS title,
               CAST(COALESCE(price_amount, '0') AS TEXT) AS price_amount,
               CAST(original_price_amount AS TEXT) AS original_price_amount,
               COALESCE(currency_code, 'CNY') AS currency_code,
               COALESCE(fulfillment_type, '') AS fulfillment_type,
               COALESCE(inventory_tracking, 'untracked') AS inventory_tracking,
               COALESCE(status, 'draft') AS status,
               NULL::text AS published_at,
               COALESCE(spec_json, '{}') AS spec_json,
               CAST(created_at AS TEXT) AS created_at,
               CAST(updated_at AS TEXT) AS updated_at
        FROM commerce_product_sku
        WHERE tenant_id = CAST($1 AS TEXT) AND organization_id = CAST($2 AS TEXT)
          AND sku_no = $3
        LIMIT 1
        "#,
    )
    .bind(tenant_id)
    .bind(organization_id)
    .bind(sku_no)
    .fetch_optional(&mut **tx)
    .await
    .map_err(|error| store_error("failed to load merchandise SKU by number", error))?;
    Ok(row.as_ref().map(crate::postgres_catalog::map_sku_row))
}

async fn load_spu_by_no(
    tx: &mut Transaction<'_, Postgres>,
    tenant_id: &str,
    organization_id: &str,
    spu_no: &str,
) -> Result<Option<SpuReplayRecord>, CommerceServiceError> {
    let row = sqlx::query_as::<_, (String, String, String, String)>(
        r#"
        SELECT CAST(id AS TEXT), CAST(spu_no AS TEXT),
               COALESCE(NULLIF(title, ''), name, CAST(id AS TEXT)) AS title,
               COALESCE(status, 'draft') AS status
        FROM commerce_product_spu
        WHERE tenant_id = CAST($1 AS TEXT) AND organization_id = CAST($2 AS TEXT)
          AND spu_no = $3
        LIMIT 1
        "#,
    )
    .bind(tenant_id)
    .bind(organization_id)
    .bind(spu_no)
    .fetch_optional(&mut **tx)
    .await
    .map_err(|error| store_error("failed to load merchandise SPU by number", error))?;
    Ok(row.map(|(id, spu_no, title, status)| SpuReplayRecord {
        id,
        spu_no,
        title,
        status,
    }))
}

async fn ensure_single_sku_spu(
    tx: &mut Transaction<'_, Postgres>,
    tenant_id: &str,
    organization_id: &str,
    spu_id: &str,
) -> Result<(), CommerceServiceError> {
    let count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM commerce_product_sku
        WHERE tenant_id = CAST($1 AS TEXT) AND organization_id = CAST($2 AS TEXT)
          AND spu_id = CAST($3 AS TEXT)
        "#,
    )
    .bind(tenant_id)
    .bind(organization_id)
    .bind(spu_id)
    .fetch_one(&mut **tx)
    .await
    .map_err(|error| store_error("failed to validate single-SKU merchandise policy", error))?;
    ensure_single_sku_count(count)
}
