use sdkwork_contract_service::{
    CommerceMoney, CommerceServiceError, CommerceSurfaceProfile,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProductType {
    Physical,
    Virtual,
    Membership,
    PointsRecharge,
    Service,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProductStatus {
    Draft,
    Active,
    Inactive,
    Archived,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FulfillmentType {
    PhysicalShipment,
    VirtualDelivery,
    MembershipActivation,
    PointsCredit,
    NoDelivery,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InventoryTrackingMode {
    Tracked,
    Untracked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductSpuDraft {
    pub tenant_id: String,
    pub organization_id: String,
    pub spu_no: String,
    pub title: String,
    pub product_type: ProductType,
    pub category_id: Option<String>,
    pub status: ProductStatus,
    pub visible_surfaces: Vec<CommerceSurfaceProfile>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductSkuDraft {
    pub tenant_id: String,
    pub organization_id: String,
    pub sku_no: String,
    pub spu_id: String,
    pub title: String,
    pub price_amount: CommerceMoney,
    pub original_price_amount: Option<CommerceMoney>,
    pub currency_code: String,
    pub fulfillment_type: FulfillmentType,
    pub inventory_tracking: InventoryTrackingMode,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductCategoryDraft {
    pub tenant_id: String,
    pub organization_id: String,
    pub category_no: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub sort_order: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductAttributeDraft {
    pub tenant_id: String,
    pub organization_id: String,
    pub attribute_no: String,
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CartItemDraft {
    pub tenant_id: String,
    pub owner_user_id: String,
    pub sku_id: String,
    pub quantity: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuyerAddressDraft {
    pub tenant_id: String,
    pub owner_user_id: String,
    pub address_id: String,
    pub receiver_name: String,
    pub receiver_phone: String,
    pub country_code: String,
    pub province: String,
    pub city: String,
    pub detail_address: String,
    pub is_default: bool,
}

impl ProductType {
    pub fn as_storage_str(&self) -> &'static str {
        match self {
            Self::Physical => "physical",
            Self::Virtual => "virtual",
            Self::Membership => "membership",
            Self::PointsRecharge => "points_recharge",
            Self::Service => "service",
        }
    }
}

impl ProductStatus {
    pub fn as_storage_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Archived => "archived",
        }
    }
}

impl FulfillmentType {
    pub fn as_storage_str(&self) -> &'static str {
        match self {
            Self::PhysicalShipment => "physical_shipment",
            Self::VirtualDelivery => "virtual_delivery",
            Self::MembershipActivation => "membership_activation",
            Self::PointsCredit => "points_credit",
            Self::NoDelivery => "no_delivery",
        }
    }
}

impl InventoryTrackingMode {
    pub fn as_storage_str(&self) -> &'static str {
        match self {
            Self::Tracked => "tracked",
            Self::Untracked => "untracked",
        }
    }
}

impl ProductSpuDraft {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tenant_id: &str,
        organization_id: &str,
        spu_no: &str,
        title: &str,
        product_type: ProductType,
        category_id: Option<&str>,
        status: ProductStatus,
        visible_surfaces: Vec<CommerceSurfaceProfile>,
    ) -> Result<Self, CommerceServiceError> {
        if visible_surfaces.is_empty() {
            return Err(CommerceServiceError::validation(
                "visible_surfaces requires at least one surface",
            ));
        }

        Ok(Self {
            tenant_id: required_text("tenant_id", tenant_id)?,
            organization_id: required_text("organization_id", organization_id)?,
            spu_no: required_text("spu_no", spu_no)?,
            title: required_text("title", title)?,
            product_type,
            category_id: optional_text(category_id),
            status,
            visible_surfaces,
        })
    }
}

impl ProductSkuDraft {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tenant_id: &str,
        organization_id: &str,
        sku_no: &str,
        spu_id: &str,
        title: &str,
        price_amount: CommerceMoney,
        original_price_amount: Option<CommerceMoney>,
        currency_code: &str,
        fulfillment_type: FulfillmentType,
        inventory_tracking: InventoryTrackingMode,
    ) -> Result<Self, CommerceServiceError> {
        Ok(Self {
            tenant_id: required_text("tenant_id", tenant_id)?,
            organization_id: required_text("organization_id", organization_id)?,
            sku_no: required_text("sku_no", sku_no)?,
            spu_id: required_text("spu_id", spu_id)?,
            title: required_text("title", title)?,
            price_amount,
            original_price_amount,
            currency_code: required_text("currency_code", currency_code)?,
            fulfillment_type,
            inventory_tracking,
        })
    }
}

impl ProductCategoryDraft {
    pub fn new(
        tenant_id: &str,
        organization_id: &str,
        category_no: &str,
        parent_id: Option<&str>,
        name: &str,
        sort_order: i64,
    ) -> Result<Self, CommerceServiceError> {
        Ok(Self {
            tenant_id: required_text("tenant_id", tenant_id)?,
            organization_id: required_text("organization_id", organization_id)?,
            category_no: required_text("category_no", category_no)?,
            parent_id: optional_text(parent_id),
            name: required_text("name", name)?,
            sort_order,
        })
    }
}

impl ProductAttributeDraft {
    pub fn new(
        tenant_id: &str,
        organization_id: &str,
        attribute_no: &str,
        name: &str,
        values: Vec<String>,
    ) -> Result<Self, CommerceServiceError> {
        let values = values
            .into_iter()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>();
        if values.is_empty() {
            return Err(CommerceServiceError::validation(
                "attribute values require at least one value",
            ));
        }

        Ok(Self {
            tenant_id: required_text("tenant_id", tenant_id)?,
            organization_id: required_text("organization_id", organization_id)?,
            attribute_no: required_text("attribute_no", attribute_no)?,
            name: required_text("name", name)?,
            values,
        })
    }
}

impl CartItemDraft {
    pub fn new(
        tenant_id: &str,
        owner_user_id: &str,
        sku_id: &str,
        quantity: u32,
    ) -> Result<Self, CommerceServiceError> {
        if quantity == 0 {
            return Err(CommerceServiceError::validation(
                "cart item quantity must be greater than zero",
            ));
        }

        Ok(Self {
            tenant_id: required_text("tenant_id", tenant_id)?,
            owner_user_id: required_text("owner_user_id", owner_user_id)?,
            sku_id: required_text("sku_id", sku_id)?,
            quantity,
        })
    }
}

impl BuyerAddressDraft {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tenant_id: &str,
        owner_user_id: &str,
        address_id: &str,
        receiver_name: &str,
        receiver_phone: &str,
        country_code: &str,
        province: &str,
        city: &str,
        detail_address: &str,
        is_default: bool,
    ) -> Result<Self, CommerceServiceError> {
        Ok(Self {
            tenant_id: required_text("tenant_id", tenant_id)?,
            owner_user_id: required_text("owner_user_id", owner_user_id)?,
            address_id: required_text("address_id", address_id)?,
            receiver_name: required_text("receiver_name", receiver_name)?,
            receiver_phone: required_text("receiver_phone", receiver_phone)?,
            country_code: required_text("country_code", country_code)?,
            province: required_text("province", province)?,
            city: required_text("city", city)?,
            detail_address: required_text("detail_address", detail_address)?,
            is_default,
        })
    }
}

fn required_text(field_name: &str, value: &str) -> Result<String, CommerceServiceError> {
    crate::validation::require_non_empty(field_name, value)?;
    Ok(value.trim().to_string())
}

fn optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}
