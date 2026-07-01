use async_trait::async_trait;
use chrono::Utc;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_merchandise_service::{CreateShopCommand, ShopProfile, ShopRepository, ShopSummary};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct SqlxShopRepository {
    pool: PgPool,
}

impl SqlxShopRepository {
    pub fn new(pool: DatabasePool) -> Self {
        Self {
            pool: pool
                .as_postgres()
                .cloned()
                .expect("merchandise repository requires postgres DatabasePool"),
        }
    }
}

#[async_trait]
impl ShopRepository for SqlxShopRepository {
    async fn find_by_tenant_and_org(
        &self,
        tenant_id: Uuid,
        organization_id: Uuid,
    ) -> Result<Option<ShopProfile>, String> {
        let row = sqlx::query(
            r#"
            SELECT id, tenant_id, organization_id, name, slug, status, created_at, updated_at
            FROM commerce_shop
            WHERE tenant_id = $1 AND organization_id = $2
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(tenant_id)
        .bind(organization_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| format!("query merchandise profile failed: {error}"))?;

        Ok(row.map(map_shop_profile))
    }

    async fn create_shop(&self, command: CreateShopCommand) -> Result<ShopProfile, String> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let status = "draft".to_owned();

        sqlx::query(
            r#"
            INSERT INTO commerce_shop (id, tenant_id, organization_id, name, slug, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(id)
        .bind(command.tenant_id)
        .bind(command.organization_id)
        .bind(&command.name)
        .bind(&command.slug)
        .bind(&status)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|error| format!("insert merchandise failed: {error}"))?;

        Ok(ShopProfile {
            id,
            tenant_id: command.tenant_id,
            organization_id: command.organization_id,
            name: command.name,
            slug: command.slug,
            status,
            created_at: now,
            updated_at: now,
        })
    }

    async fn list_for_admin(&self, tenant_id: Uuid) -> Result<Vec<ShopSummary>, String> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, status
            FROM commerce_shop
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| format!("list shops failed: {error}"))?;

        Ok(rows
            .into_iter()
            .map(|row| ShopSummary {
                id: row.get("id"),
                name: row.get("name"),
                status: row.get("status"),
            })
            .collect())
    }
}

fn map_shop_profile(row: sqlx::postgres::PgRow) -> ShopProfile {
    ShopProfile {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        name: row.get("name"),
        slug: row.get("slug"),
        status: row.get("status"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_type_name_is_stable() {
        let _ = std::any::type_name::<SqlxShopRepository>();
    }
}
