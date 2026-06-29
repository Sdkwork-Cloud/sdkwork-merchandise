use uuid::Uuid;

use super::domain::{CreateShopCommand, ShopProfile, ShopSummary};
use super::ports::ShopRepository;

pub struct ShopService<R: ShopRepository> {
    repository: R,
}

impl<R: ShopRepository> ShopService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn retrieve_my_shop(
        &self,
        tenant_id: Uuid,
        organization_id: Uuid,
    ) -> Result<Option<ShopProfile>, String> {
        self.repository
            .find_by_tenant_and_org(tenant_id, organization_id)
            .await
    }

    pub async fn create_shop(&self, command: CreateShopCommand) -> Result<ShopProfile, String> {
        if command.name.trim().is_empty() || command.slug.trim().is_empty() {
            return Err("shop name and slug are required".to_owned());
        }
        self.repository.create_shop(command).await
    }

    pub async fn list_shops_for_admin(&self, tenant_id: Uuid) -> Result<Vec<ShopSummary>, String> {
        self.repository.list_for_admin(tenant_id).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;
    use chrono::Utc;

    use super::*;
    use crate::runtime::domain::ShopProfile;

    struct MemoryShopRepository {
        shops: Mutex<Vec<ShopProfile>>,
    }

    impl MemoryShopRepository {
        fn new() -> Self {
            Self {
                shops: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ShopRepository for MemoryShopRepository {
        async fn find_by_tenant_and_org(
            &self,
            tenant_id: Uuid,
            organization_id: Uuid,
        ) -> Result<Option<ShopProfile>, String> {
            Ok(self
                .shops
                .lock()
                .map_err(|_| "lock poisoned".to_owned())?
                .iter()
                .find(|shop| shop.tenant_id == tenant_id && shop.organization_id == organization_id)
                .cloned())
        }

        async fn create_shop(&self, command: CreateShopCommand) -> Result<ShopProfile, String> {
            let now = Utc::now();
            let profile = ShopProfile {
                id: Uuid::new_v4(),
                tenant_id: command.tenant_id,
                organization_id: command.organization_id,
                name: command.name,
                slug: command.slug,
                status: "draft".to_owned(),
                created_at: now,
                updated_at: now,
            };
            self.shops
                .lock()
                .map_err(|_| "lock poisoned".to_owned())?
                .push(profile.clone());
            Ok(profile)
        }

        async fn list_for_admin(&self, tenant_id: Uuid) -> Result<Vec<ShopSummary>, String> {
            Ok(self
                .shops
                .lock()
                .map_err(|_| "lock poisoned".to_owned())?
                .iter()
                .filter(|shop| shop.tenant_id == tenant_id)
                .map(|shop| ShopSummary {
                    id: shop.id,
                    name: shop.name.clone(),
                    status: shop.status.clone(),
                })
                .collect())
        }
    }

    #[tokio::test]
    async fn shop_service_rejects_empty_name() {
        let service = ShopService::new(MemoryShopRepository::new());
        let result = service
            .create_shop(CreateShopCommand {
                tenant_id: Uuid::new_v4(),
                organization_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                name: "   ".to_owned(),
                slug: "demo".to_owned(),
            })
            .await;
        assert!(result.is_err());
    }
}
