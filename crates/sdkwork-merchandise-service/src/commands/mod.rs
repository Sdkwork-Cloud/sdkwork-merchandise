use sdkwork_contract_service::{CommerceMoney, CommerceServiceError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateCategoryCommand {
    pub tenant_id: String,
    pub organization_id: String,
    pub category_no: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub sort_order: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateCategoryCommand {
    pub tenant_id: String,
    pub category_id: String,
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub sort_order: Option<i64>,
    pub status: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeleteCategoryCommand {
    pub tenant_id: String,
    pub category_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateAttributeCommand {
    pub tenant_id: String,
    pub organization_id: String,
    pub attribute_no: String,
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreatePriceListCommand {
    pub tenant_id: String,
    pub organization_id: String,
    pub price_list_no: String,
    pub currency_code: String,
    pub market_code: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdatePriceListCommand {
    pub tenant_id: String,
    pub price_list_id: String,
    pub status: Option<String>,
    pub starts_at: Option<String>,
    pub ends_at: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateProductSpuCommand {
    pub tenant_id: String,
    pub organization_id: String,
    pub spu_no: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub product_type: String,
    pub category_id: Option<String>,
    pub visible_surfaces: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateProductSpuCommand {
    pub tenant_id: String,
    pub spu_id: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<String>,
    pub visible_surfaces: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeleteProductSpuCommand {
    pub tenant_id: String,
    pub spu_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublishSpuCommand {
    pub tenant_id: String,
    pub spu_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchiveSpuCommand {
    pub tenant_id: String,
    pub spu_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateProductSkuCommand {
    pub tenant_id: String,
    pub organization_id: String,
    pub spu_id: String,
    pub sku_no: String,
    pub name: String,
    pub title: String,
    pub price_amount: CommerceMoney,
    pub original_price_amount: Option<CommerceMoney>,
    pub currency_code: String,
    pub fulfillment_type: String,
    pub inventory_tracking: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateProductSkuCommand {
    pub tenant_id: String,
    pub sku_id: String,
    pub name: Option<String>,
    pub title: Option<String>,
    pub price_amount: Option<CommerceMoney>,
    pub original_price_amount: Option<CommerceMoney>,
    pub currency_code: Option<String>,
    pub fulfillment_type: Option<String>,
    pub inventory_tracking: Option<String>,
    pub status: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeleteProductSkuCommand {
    pub tenant_id: String,
    pub sku_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateCategoryAttributeCommand {
    pub tenant_id: String,
    pub organization_id: String,
    pub category_id: String,
    pub attribute_id: String,
    pub required: bool,
    pub searchable: bool,
    pub filterable: bool,
    pub sort_order: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateCategoryAttributeCommand {
    pub tenant_id: String,
    pub binding_id: String,
    pub required: Option<bool>,
    pub searchable: Option<bool>,
    pub filterable: Option<bool>,
    pub sort_order: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeleteCategoryAttributeCommand {
    pub tenant_id: String,
    pub binding_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddCartItemCommand {
    pub tenant_id: String,
    pub owner_user_id: String,
    pub sku_id: String,
    pub quantity: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateCartItemCommand {
    pub tenant_id: String,
    pub owner_user_id: String,
    pub cart_item_id: String,
    pub quantity: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoveCartItemCommand {
    pub tenant_id: String,
    pub owner_user_id: String,
    pub cart_item_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateAddressCommand {
    pub tenant_id: String,
    pub owner_user_id: String,
    pub receiver_name: String,
    pub receiver_phone: String,
    pub country_code: String,
    pub province: String,
    pub city: String,
    pub detail_address: String,
    pub is_default: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateAddressCommand {
    pub tenant_id: String,
    pub owner_user_id: String,
    pub address_id: String,
    pub receiver_name: Option<String>,
    pub receiver_phone: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub detail_address: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeleteAddressCommand {
    pub tenant_id: String,
    pub owner_user_id: String,
    pub address_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetDefaultAddressCommand {
    pub tenant_id: String,
    pub owner_user_id: String,
    pub address_id: String,
}

macro_rules! impl_required_text_command {
    ($cmd:ident, $($field:ident),+) => {
        impl $cmd {
            pub fn validate(&self) -> Result<(), CommerceServiceError> {
                $(
                    crate::validation::require_non_empty(stringify!($field), &self.$field)?;
                )+
                Ok(())
            }
        }
    };
}

impl_required_text_command!(
    CreateCategoryCommand,
    tenant_id,
    organization_id,
    category_no,
    name
);
impl_required_text_command!(UpdateCategoryCommand, tenant_id, category_id);
impl_required_text_command!(DeleteCategoryCommand, tenant_id, category_id);
impl_required_text_command!(
    CreateAttributeCommand,
    tenant_id,
    organization_id,
    attribute_no,
    name
);
impl_required_text_command!(
    CreatePriceListCommand,
    tenant_id,
    organization_id,
    price_list_no,
    currency_code
);
impl_required_text_command!(UpdatePriceListCommand, tenant_id, price_list_id);
impl_required_text_command!(
    CreateProductSpuCommand,
    tenant_id,
    organization_id,
    spu_no,
    title,
    product_type,
    visible_surfaces
);
impl_required_text_command!(UpdateProductSpuCommand, tenant_id, spu_id);
impl_required_text_command!(DeleteProductSpuCommand, tenant_id, spu_id);
impl_required_text_command!(PublishSpuCommand, tenant_id, spu_id);
impl_required_text_command!(ArchiveSpuCommand, tenant_id, spu_id);
impl_required_text_command!(
    CreateProductSkuCommand,
    tenant_id,
    organization_id,
    spu_id,
    sku_no,
    name,
    title,
    currency_code,
    fulfillment_type,
    inventory_tracking
);
impl_required_text_command!(UpdateProductSkuCommand, tenant_id, sku_id);
impl_required_text_command!(DeleteProductSkuCommand, tenant_id, sku_id);
impl_required_text_command!(
    CreateCategoryAttributeCommand,
    tenant_id,
    organization_id,
    category_id,
    attribute_id
);
impl_required_text_command!(UpdateCategoryAttributeCommand, tenant_id, binding_id);
impl_required_text_command!(DeleteCategoryAttributeCommand, tenant_id, binding_id);
impl_required_text_command!(AddCartItemCommand, tenant_id, owner_user_id, sku_id);
impl_required_text_command!(
    UpdateCartItemCommand,
    tenant_id,
    owner_user_id,
    cart_item_id
);
impl_required_text_command!(
    RemoveCartItemCommand,
    tenant_id,
    owner_user_id,
    cart_item_id
);
impl_required_text_command!(
    CreateAddressCommand,
    tenant_id,
    owner_user_id,
    receiver_name,
    receiver_phone,
    country_code,
    province,
    city,
    detail_address
);
impl_required_text_command!(UpdateAddressCommand, tenant_id, owner_user_id, address_id);
impl_required_text_command!(DeleteAddressCommand, tenant_id, owner_user_id, address_id);
impl_required_text_command!(
    SetDefaultAddressCommand,
    tenant_id,
    owner_user_id,
    address_id
);
