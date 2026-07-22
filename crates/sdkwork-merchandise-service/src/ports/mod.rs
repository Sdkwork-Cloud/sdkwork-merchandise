use crate::{commands::*, queries::*};
use sdkwork_contract_service::CommerceServiceError;

mod single_sku_merchandise;

pub use single_sku_merchandise::*;

pub const CATALOG_REPOSITORY_PORT: &str = "catalog.repository";
pub const CART_REPOSITORY_PORT: &str = "cart.repository";
pub const BUYER_ADDRESS_REPOSITORY_PORT: &str = "buyer_address.repository";
pub const IDEMPOTENCY_REPOSITORY_PORT: &str = "idempotency.repository";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CatalogRepositoryCommand {
    CreateCategory,
    CreateAttribute,
    CreateSpu,
    CreateSku,
    AddCartItem,
    RemoveCartItem,
    UpsertBuyerAddress,
}

pub struct CatalogPortRequirement;

impl CatalogPortRequirement {
    pub fn standard_commands() -> Vec<CatalogRepositoryCommand> {
        vec![
            CatalogRepositoryCommand::CreateCategory,
            CatalogRepositoryCommand::CreateAttribute,
            CatalogRepositoryCommand::CreateSpu,
            CatalogRepositoryCommand::CreateSku,
            CatalogRepositoryCommand::AddCartItem,
            CatalogRepositoryCommand::RemoveCartItem,
            CatalogRepositoryCommand::UpsertBuyerAddress,
        ]
    }
}

#[derive(Clone, Debug)]
pub struct CategoryRecord {
    pub id: String,
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub category_no: String,
    pub parent_id: Option<String>,
    pub path: String,
    pub level_no: i64,
    pub name: String,
    pub sort_order: i64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct AttributeRecord {
    pub id: String,
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub attribute_no: String,
    pub name: String,
    pub value_type: String,
    pub scope: String,
    pub status: String,
    pub sort_order: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct AttributeValueRecord {
    pub id: String,
    pub tenant_id: String,
    pub attribute_id: String,
    pub value_code: String,
    pub display_value: String,
    pub sort_order: i64,
    pub status: String,
}

#[derive(Clone, Debug)]
pub struct SpuRecord {
    pub id: String,
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub spu_no: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub product_type: String,
    pub category_id: Option<String>,
    pub status: String,
    pub published_at: Option<String>,
    pub visible_surfaces: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct SkuRecord {
    pub id: String,
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub spu_id: String,
    pub sku_no: String,
    pub name: String,
    pub title: String,
    pub price_amount: String,
    pub original_price_amount: Option<String>,
    pub currency_code: String,
    pub fulfillment_type: String,
    pub inventory_tracking: String,
    pub status: String,
    pub published_at: Option<String>,
    pub spec_json: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct PriceListRecord {
    pub id: String,
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub price_list_no: String,
    pub currency_code: String,
    pub market_code: Option<String>,
    pub status: String,
    pub starts_at: Option<String>,
    pub ends_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct PriceListItemRecord {
    pub id: String,
    pub tenant_id: String,
    pub price_list_id: String,
    pub sku_id: String,
    pub price_amount: String,
    pub currency_code: String,
}

#[derive(Clone, Debug)]
pub struct CategoryAttributeRecord {
    pub id: String,
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub category_id: String,
    pub attribute_id: String,
    pub required: bool,
    pub searchable: bool,
    pub filterable: bool,
    pub sort_order: i64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct CartItemRecord {
    pub id: String,
    pub tenant_id: String,
    pub owner_user_id: String,
    pub sku_id: String,
    pub quantity: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct AddressRecord {
    pub id: String,
    pub tenant_id: String,
    pub owner_user_id: String,
    pub receiver_name: String,
    pub receiver_phone: String,
    pub country_code: String,
    pub province: String,
    pub city: String,
    pub detail_address: String,
    pub is_default: bool,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

pub trait CatalogRepositoryPort: Send + Sync {
    fn list_categories(
        &self,
        query: &CategoryListQuery,
    ) -> Result<Vec<CategoryRecord>, CommerceServiceError>;

    fn create_category(
        &self,
        command: &CreateCategoryCommand,
    ) -> Result<CategoryRecord, CommerceServiceError>;

    fn update_category(
        &self,
        command: &UpdateCategoryCommand,
    ) -> Result<CategoryRecord, CommerceServiceError>;

    fn delete_category(&self, command: &DeleteCategoryCommand) -> Result<(), CommerceServiceError>;

    fn list_attributes(
        &self,
        query: &AttributeListQuery,
    ) -> Result<Vec<AttributeRecord>, CommerceServiceError>;

    fn create_attribute(
        &self,
        command: &CreateAttributeCommand,
    ) -> Result<AttributeRecord, CommerceServiceError>;

    fn list_price_lists(
        &self,
        query: &PriceListListQuery,
    ) -> Result<Vec<PriceListRecord>, CommerceServiceError>;

    fn create_price_list(
        &self,
        command: &CreatePriceListCommand,
    ) -> Result<PriceListRecord, CommerceServiceError>;

    fn update_price_list(
        &self,
        command: &UpdatePriceListCommand,
    ) -> Result<PriceListRecord, CommerceServiceError>;

    fn list_category_attributes(
        &self,
        query: &CategoryAttributeListQuery,
    ) -> Result<Vec<CategoryAttributeRecord>, CommerceServiceError>;

    fn create_category_attribute(
        &self,
        command: &CreateCategoryAttributeCommand,
    ) -> Result<CategoryAttributeRecord, CommerceServiceError>;

    fn update_category_attribute(
        &self,
        command: &UpdateCategoryAttributeCommand,
    ) -> Result<CategoryAttributeRecord, CommerceServiceError>;

    fn delete_category_attribute(
        &self,
        command: &DeleteCategoryAttributeCommand,
    ) -> Result<(), CommerceServiceError>;

    fn list_spus(
        &self,
        query: &ProductSpuListQuery,
    ) -> Result<Vec<SpuRecord>, CommerceServiceError>;

    fn retrieve_spu(
        &self,
        query: &ProductSpuRetrieveQuery,
    ) -> Result<Option<SpuRecord>, CommerceServiceError>;

    fn create_spu(
        &self,
        command: &CreateProductSpuCommand,
    ) -> Result<SpuRecord, CommerceServiceError>;

    fn update_spu(
        &self,
        command: &UpdateProductSpuCommand,
    ) -> Result<SpuRecord, CommerceServiceError>;

    fn delete_spu(&self, command: &DeleteProductSpuCommand) -> Result<(), CommerceServiceError>;

    fn publish_spu(&self, command: &PublishSpuCommand) -> Result<SpuRecord, CommerceServiceError>;

    fn archive_spu(&self, command: &ArchiveSpuCommand) -> Result<SpuRecord, CommerceServiceError>;

    fn list_skus(
        &self,
        query: &ProductSkuListQuery,
    ) -> Result<Vec<SkuRecord>, CommerceServiceError>;

    fn retrieve_sku(
        &self,
        query: &ProductSkuRetrieveQuery,
    ) -> Result<Option<SkuRecord>, CommerceServiceError>;

    fn create_sku(
        &self,
        command: &CreateProductSkuCommand,
    ) -> Result<SkuRecord, CommerceServiceError>;

    fn update_sku(
        &self,
        command: &UpdateProductSkuCommand,
    ) -> Result<SkuRecord, CommerceServiceError>;

    fn delete_sku(&self, command: &DeleteProductSkuCommand) -> Result<(), CommerceServiceError>;

    fn list_cart_items(
        &self,
        query: &CartRetrieveQuery,
    ) -> Result<Vec<CartItemRecord>, CommerceServiceError>;

    fn add_cart_item(
        &self,
        command: &AddCartItemCommand,
    ) -> Result<CartItemRecord, CommerceServiceError>;

    fn update_cart_item(
        &self,
        command: &UpdateCartItemCommand,
    ) -> Result<CartItemRecord, CommerceServiceError>;

    fn remove_cart_item(&self, command: &RemoveCartItemCommand)
        -> Result<(), CommerceServiceError>;

    fn list_addresses(
        &self,
        query: &AddressListQuery,
    ) -> Result<Vec<AddressRecord>, CommerceServiceError>;

    fn create_address(
        &self,
        command: &CreateAddressCommand,
    ) -> Result<AddressRecord, CommerceServiceError>;

    fn update_address(
        &self,
        command: &UpdateAddressCommand,
    ) -> Result<AddressRecord, CommerceServiceError>;

    fn delete_address(&self, command: &DeleteAddressCommand) -> Result<(), CommerceServiceError>;

    fn set_default_address(
        &self,
        command: &SetDefaultAddressCommand,
    ) -> Result<AddressRecord, CommerceServiceError>;
}
