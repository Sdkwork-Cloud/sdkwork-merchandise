use async_trait::async_trait;
use uuid::Uuid;

use super::domain::{CreateShopCommand, ShopProfile, ShopSummary};

#[async_trait]
pub trait ShopRepository: Send + Sync {
    async fn find_by_tenant_and_org(
        &self,
        tenant_id: Uuid,
        organization_id: Uuid,
    ) -> Result<Option<ShopProfile>, String>;

    async fn create_shop(&self, command: CreateShopCommand) -> Result<ShopProfile, String>;

    async fn list_for_admin(&self, tenant_id: Uuid) -> Result<Vec<ShopSummary>, String>;
}
