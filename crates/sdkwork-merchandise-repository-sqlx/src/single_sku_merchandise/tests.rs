use std::sync::{
    atomic::{AtomicI64, Ordering},
    Arc,
};

use sdkwork_database_id::{IdGenError, IdGenerator};
use sdkwork_database_sqlx::{DatabasePool, PoolContext};
use sdkwork_merchandise_service::{
    description, public_spec, CreateSingleSkuMerchandiseCommand, SingleSkuMerchandiseListQuery,
    SingleSkuMerchandiseRepositoryPort, UpdateSingleSkuMerchandiseCommand,
};
use sqlx::sqlite::SqlitePoolOptions;

use super::SqlxSingleSkuMerchandiseRepository;

#[derive(Debug)]
struct TestIdGenerator {
    next: AtomicI64,
}

impl TestIdGenerator {
    fn new(first: i64) -> Self {
        Self {
            next: AtomicI64::new(first),
        }
    }
}

impl IdGenerator for TestIdGenerator {
    fn next_id(&self) -> Result<String, IdGenError> {
        Ok(self.next.fetch_add(1, Ordering::Relaxed).to_string())
    }

    fn label(&self) -> &str {
        "test-snowflake"
    }
}

async fn repository_with_first_id(first_id: i64) -> SqlxSingleSkuMerchandiseRepository {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("sqlite");
    sqlx::raw_sql(
        r#"
        CREATE TABLE commerce_product_spu (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            organization_id TEXT NOT NULL,
            spu_no TEXT NOT NULL,
            name TEXT,
            title TEXT,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            UNIQUE (tenant_id, organization_id, spu_no)
        );
        CREATE TABLE commerce_product_sku (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            organization_id TEXT NOT NULL,
            spu_id TEXT NOT NULL,
            sku_no TEXT NOT NULL,
            name TEXT,
            title TEXT,
            price_amount TEXT,
            original_price_amount TEXT,
            currency_code TEXT,
            fulfillment_type TEXT,
            inventory_tracking TEXT,
            status TEXT NOT NULL,
            spec_json TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            UNIQUE (tenant_id, organization_id, sku_no)
        );
        "#,
    )
    .execute(&pool)
    .await
    .expect("schema");
    SqlxSingleSkuMerchandiseRepository::new(
        DatabasePool::Sqlite(
            pool,
            PoolContext {
                config: Default::default(),
            },
        ),
        Arc::new(TestIdGenerator::new(first_id)),
    )
}

async fn repository() -> SqlxSingleSkuMerchandiseRepository {
    repository_with_first_id(1_000_000).await
}

fn create_command(idempotency_key: &str, title: &str) -> CreateSingleSkuMerchandiseCommand {
    CreateSingleSkuMerchandiseCommand::new(
        "tenant-a",
        "org-a",
        "notary",
        "notary",
        title,
        Some("Three-day service"),
        "100",
        Some("120"),
        "CNY",
        "active",
        serde_json::json!({"durationDays": 3}),
        idempotency_key,
    )
    .expect("command")
}

#[tokio::test]
async fn create_is_atomic_and_idempotent() {
    let repository = repository().await;
    let first = repository
        .create_single_sku(create_command("idem-1", "Remote notarization"))
        .await
        .expect("create");
    let replay = repository
        .create_single_sku(create_command("idem-1", "Remote notarization"))
        .await
        .expect("replay");
    assert_eq!(first.id, replay.id);
    assert!(first.id.parse::<i64>().is_ok_and(|value| value > 0));
    assert!(first.spu_id.parse::<i64>().is_ok_and(|value| value > 0));
    assert_eq!(description(&replay).as_deref(), Some("Three-day service"));
    assert_eq!(public_spec(&replay), serde_json::json!({"durationDays": 3}));

    let conflict = repository
        .create_single_sku(create_command("idem-1", "Different title"))
        .await
        .expect_err("payload conflict");
    assert_eq!(conflict.code(), "conflict");
}

#[tokio::test]
async fn create_fails_closed_for_non_snowflake_ids() {
    let repository = repository_with_first_id(0).await;
    let error = repository
        .create_single_sku(create_command("idem-1", "Remote notarization"))
        .await
        .expect_err("invalid id provider");
    assert_eq!(error.code(), "storage");

    let pool = repository.database_pool().as_sqlite().expect("sqlite");
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM commerce_product_spu")
        .fetch_one(pool)
        .await
        .expect("count");
    assert_eq!(count, 0);
}

#[tokio::test]
async fn list_filters_and_paginates_at_the_store() {
    let repository = repository().await;
    repository
        .create_single_sku(create_command("idem-1", "Remote notarization"))
        .await
        .expect("first");
    repository
        .create_single_sku(create_command("idem-2", "Document notarization"))
        .await
        .expect("second");

    let page = repository
        .list_skus(
            SingleSkuMerchandiseListQuery::new(
                "tenant-a",
                Some("org-a"),
                "notary",
                Some("notarization"),
                Some("active"),
                1,
                0,
            )
            .expect("query"),
        )
        .await
        .expect("list");
    assert_eq!(page.items.len(), 1);
    assert!(page.has_more);
    assert_eq!(page.next_offset, Some(1));
}

#[tokio::test]
async fn update_keeps_spu_and_sku_in_sync() {
    let repository = repository().await;
    let created = repository
        .create_single_sku(create_command("idem-1", "Remote notarization"))
        .await
        .expect("create");
    let updated = repository
        .update_single_sku(
            UpdateSingleSkuMerchandiseCommand::new(
                "tenant-a",
                "org-a",
                &created.id,
                "notary",
                Some("notary"),
                Some("Updated notarization"),
                Some(Some("Updated description")),
                Some("150"),
                Some(Some("180")),
                Some("CNY"),
                Some("inactive"),
                Some(serde_json::json!({"durationDays": 5})),
            )
            .expect("update command"),
        )
        .await
        .expect("update");
    assert_eq!(updated.title, "Updated notarization");
    assert_eq!(updated.price_amount, "150");
    assert_eq!(updated.status, "inactive");
    assert_eq!(
        description(&updated).as_deref(),
        Some("Updated description")
    );
    assert_eq!(
        public_spec(&updated),
        serde_json::json!({"durationDays": 5})
    );
}

#[tokio::test]
async fn update_distinguishes_omitted_and_cleared_nullable_fields() {
    let repository = repository().await;
    let created = repository
        .create_single_sku(create_command("idem-1", "Remote notarization"))
        .await
        .expect("create");

    let preserved = repository
        .update_single_sku(
            UpdateSingleSkuMerchandiseCommand::new(
                "tenant-a",
                "org-a",
                &created.id,
                "notary",
                None,
                Some("Preserved nullable fields"),
                None,
                None,
                None,
                None,
                None,
                Some(serde_json::json!({"durationDays": 4})),
            )
            .expect("omission update command"),
        )
        .await
        .expect("omit nullable fields");
    assert_eq!(
        description(&preserved).as_deref(),
        Some("Three-day service")
    );
    assert_eq!(preserved.original_price_amount.as_deref(), Some("120"));
    assert_eq!(
        public_spec(&preserved),
        serde_json::json!({"durationDays": 4})
    );

    let cleared = repository
        .update_single_sku(
            UpdateSingleSkuMerchandiseCommand::new(
                "tenant-a",
                "org-a",
                &created.id,
                "notary",
                None,
                None,
                Some(None),
                None,
                Some(None),
                None,
                None,
                None,
            )
            .expect("clear update command"),
        )
        .await
        .expect("clear nullable fields");
    assert_eq!(description(&cleared), None);
    assert_eq!(cleared.original_price_amount, None);
}

#[tokio::test]
async fn update_rejects_a_spu_shared_by_multiple_skus() {
    let repository = repository().await;
    let created = repository
        .create_single_sku(create_command("idem-1", "Remote notarization"))
        .await
        .expect("create");
    let pool = repository.database_pool().as_sqlite().expect("sqlite");
    sqlx::query(
        r#"
        INSERT INTO commerce_product_sku
            (id, tenant_id, organization_id, spu_id, sku_no, name, title, price_amount,
             original_price_amount, currency_code, fulfillment_type, inventory_tracking,
             status, spec_json, created_at, updated_at)
        VALUES ('2000000', 'tenant-a', 'org-a', ?1, 'shared-sku', 'Shared', 'Shared',
                '100', NULL, 'CNY', 'notary', 'untracked', 'active', '{}',
                CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        "#,
    )
    .bind(&created.spu_id)
    .execute(pool)
    .await
    .expect("second sku");

    let error = repository
        .update_single_sku(
            UpdateSingleSkuMerchandiseCommand::new(
                "tenant-a",
                "org-a",
                &created.id,
                "notary",
                None,
                Some("Rejected update"),
                None,
                None,
                None,
                None,
                Some("inactive"),
                None,
            )
            .expect("update command"),
        )
        .await
        .expect_err("shared SPU conflict");
    assert_eq!(error.code(), "conflict");
}
