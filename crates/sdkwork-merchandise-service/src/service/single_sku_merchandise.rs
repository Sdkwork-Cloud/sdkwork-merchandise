use sdkwork_contract_service::CommerceServiceError;

use crate::{
    CreateSingleSkuMerchandiseCommand, SingleSkuMerchandiseListQuery, SingleSkuMerchandisePage,
    SingleSkuMerchandiseRepositoryPort, SkuRecord, UpdateSingleSkuMerchandiseCommand,
};

pub struct SingleSkuMerchandiseService<Repository> {
    repository: Repository,
}

impl<Repository> SingleSkuMerchandiseService<Repository>
where
    Repository: SingleSkuMerchandiseRepositoryPort,
{
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }

    pub fn repository(&self) -> &Repository {
        &self.repository
    }

    pub async fn list(
        &self,
        query: SingleSkuMerchandiseListQuery,
    ) -> Result<SingleSkuMerchandisePage, CommerceServiceError> {
        self.repository.list_skus(query).await
    }

    pub async fn create(
        &self,
        command: CreateSingleSkuMerchandiseCommand,
    ) -> Result<SkuRecord, CommerceServiceError> {
        self.repository.create_single_sku(command).await
    }

    pub async fn update(
        &self,
        command: UpdateSingleSkuMerchandiseCommand,
    ) -> Result<SkuRecord, CommerceServiceError> {
        self.repository.update_single_sku(command).await
    }
}
