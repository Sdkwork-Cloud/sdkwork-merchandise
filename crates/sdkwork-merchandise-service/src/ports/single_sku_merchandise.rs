use async_trait::async_trait;
use sdkwork_contract_service::{CommerceMoney, CommerceServiceError};
use sdkwork_utils_rust::{sha256_hash, slugify};
use serde_json::{Map, Value};

use super::SkuRecord;

/// Stable owner boundary for capabilities that model one sellable item as one
/// SPU with one SKU. Consumers select their own capability with
/// `fulfillment_type`; the merchandise owner does not depend on those domains.
pub const SINGLE_SKU_MERCHANDISE_REPOSITORY_PORT: &str = "merchandise.single_sku.repository";
pub const SINGLE_SKU_MERCHANDISE_DESCRIPTION_KEY: &str = "description";
pub const SINGLE_SKU_MERCHANDISE_PRODUCT_TYPE_KEY: &str = "productType";
pub const SINGLE_SKU_MERCHANDISE_POLICY_KEY: &str = "skuPolicy";
pub const SINGLE_SKU_MERCHANDISE_METADATA_KEY: &str = "_sdkwork";
pub const SINGLE_SKU_MERCHANDISE_MAX_PAGE_SIZE: i64 = 200;

/// Nullable PATCH input where `None` means omitted, `Some(Some(value))` means
/// set, and `Some(None)` means explicitly clear the persisted value.
pub type NullablePatch<T> = Option<Option<T>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SingleSkuMerchandiseListQuery {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub fulfillment_type: String,
    pub search_term: Option<String>,
    pub status: Option<String>,
    pub page_size: i64,
    pub offset: i64,
}

impl SingleSkuMerchandiseListQuery {
    pub fn new(
        tenant_id: &str,
        organization_id: Option<&str>,
        fulfillment_type: &str,
        search_term: Option<&str>,
        status: Option<&str>,
        page_size: i64,
        offset: i64,
    ) -> Result<Self, CommerceServiceError> {
        let tenant_id = required_text("tenant_id", tenant_id)?;
        let fulfillment_type = required_text("fulfillment_type", fulfillment_type)?;
        if !(1..=SINGLE_SKU_MERCHANDISE_MAX_PAGE_SIZE).contains(&page_size) {
            return Err(CommerceServiceError::validation(
                "page_size must be between 1 and 200",
            ));
        }
        if offset < 0 {
            return Err(CommerceServiceError::validation(
                "offset must be greater than or equal to zero",
            ));
        }
        if offset.checked_add(page_size).is_none() {
            return Err(CommerceServiceError::validation(
                "offset and page_size exceed the supported range",
            ));
        }

        Ok(Self {
            tenant_id,
            organization_id: optional_text(organization_id),
            fulfillment_type: fulfillment_type.to_ascii_lowercase(),
            search_term: optional_text(search_term),
            status: optional_text(status).map(|value| value.to_ascii_lowercase()),
            page_size,
            offset,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateSingleSkuMerchandiseCommand {
    pub tenant_id: String,
    pub organization_id: String,
    pub product_type: String,
    pub fulfillment_type: String,
    pub title: String,
    pub description: Option<String>,
    pub price_amount: CommerceMoney,
    pub original_price_amount: Option<CommerceMoney>,
    pub currency_code: String,
    pub status: String,
    pub spec: Value,
    pub idempotency_key: String,
}

impl CreateSingleSkuMerchandiseCommand {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tenant_id: &str,
        organization_id: &str,
        product_type: &str,
        fulfillment_type: &str,
        title: &str,
        description: Option<&str>,
        price_amount: &str,
        original_price_amount: Option<&str>,
        currency_code: &str,
        status: &str,
        spec: Value,
        idempotency_key: &str,
    ) -> Result<Self, CommerceServiceError> {
        let title = required_text("title", title)?;
        let product_type = required_text("product_type", product_type)?;
        let fulfillment_type = required_text("fulfillment_type", fulfillment_type)?;
        let currency_code = normalize_currency_code(currency_code)?;
        let status = normalize_status(status)?;
        let spec = normalize_public_spec(spec)?;
        Ok(Self {
            tenant_id: required_text("tenant_id", tenant_id)?,
            organization_id: required_text("organization_id", organization_id)?,
            product_type,
            fulfillment_type: fulfillment_type.to_ascii_lowercase(),
            title,
            description: optional_text(description),
            price_amount: CommerceMoney::new(price_amount.trim())
                .map_err(CommerceServiceError::validation)?,
            original_price_amount: original_price_amount
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| CommerceMoney::new(value).map_err(CommerceServiceError::validation))
                .transpose()?,
            currency_code,
            status,
            spec,
            idempotency_key: required_text("idempotency_key", idempotency_key)?,
        })
    }

    pub fn stable_identity(&self) -> SingleSkuMerchandiseIdentity {
        stable_identity(
            &self.tenant_id,
            &self.organization_id,
            &self.fulfillment_type,
            &self.idempotency_key,
        )
    }

    pub fn persisted_spec(&self) -> Result<Value, CommerceServiceError> {
        let mut spec = normalize_public_spec(self.spec.clone())?;
        let object = spec
            .as_object_mut()
            .ok_or_else(|| CommerceServiceError::validation("spec must be a JSON object"))?;
        let metadata = object
            .entry(SINGLE_SKU_MERCHANDISE_METADATA_KEY)
            .or_insert_with(|| Value::Object(Map::new()));
        let metadata = metadata.as_object_mut().ok_or_else(|| {
            CommerceServiceError::validation("reserved merchandise metadata must be an object")
        })?;
        metadata.insert(
            SINGLE_SKU_MERCHANDISE_PRODUCT_TYPE_KEY.to_owned(),
            Value::String(self.product_type.clone()),
        );
        metadata.insert(
            SINGLE_SKU_MERCHANDISE_POLICY_KEY.to_owned(),
            Value::String("one_spu_one_sku".to_owned()),
        );
        if let Some(description) = &self.description {
            metadata.insert(
                SINGLE_SKU_MERCHANDISE_DESCRIPTION_KEY.to_owned(),
                Value::String(description.clone()),
            );
        }
        Ok(spec)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateSingleSkuMerchandiseCommand {
    pub tenant_id: String,
    pub organization_id: String,
    pub sku_id: String,
    pub product_type: Option<String>,
    pub fulfillment_type: String,
    pub title: Option<String>,
    pub description: NullablePatch<String>,
    pub price_amount: Option<CommerceMoney>,
    pub original_price_amount: NullablePatch<CommerceMoney>,
    pub currency_code: Option<String>,
    pub status: Option<String>,
    pub spec: Option<Value>,
}

impl UpdateSingleSkuMerchandiseCommand {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tenant_id: &str,
        organization_id: &str,
        sku_id: &str,
        fulfillment_type: &str,
        product_type: Option<&str>,
        title: Option<&str>,
        description: NullablePatch<&str>,
        price_amount: Option<&str>,
        original_price_amount: NullablePatch<&str>,
        currency_code: Option<&str>,
        status: Option<&str>,
        spec: Option<Value>,
    ) -> Result<Self, CommerceServiceError> {
        let fulfillment_type = required_text("fulfillment_type", fulfillment_type)?;
        let spec = spec.map(normalize_public_spec).transpose()?;
        Ok(Self {
            tenant_id: required_text("tenant_id", tenant_id)?,
            organization_id: required_text("organization_id", organization_id)?,
            sku_id: required_text("sku_id", sku_id)?,
            product_type: product_type
                .map(|value| required_text("product_type", value))
                .transpose()?,
            fulfillment_type: fulfillment_type.to_ascii_lowercase(),
            title: title
                .map(|value| required_text("title", value))
                .transpose()?,
            description: parse_nullable_text_patch("description", description)?,
            price_amount: parse_optional_money("price_amount", price_amount)?,
            original_price_amount: parse_nullable_money_patch(
                "original_price_amount",
                original_price_amount,
            )?,
            currency_code: currency_code.map(normalize_currency_code).transpose()?,
            status: status.map(normalize_status).transpose()?,
            spec,
        })
    }
}

#[derive(Clone, Debug)]
pub struct SingleSkuMerchandisePage {
    pub items: Vec<SkuRecord>,
    pub has_more: bool,
    pub next_offset: Option<i64>,
}

/// Deterministic business numbers used to find idempotent SPU/SKU replays.
/// Database primary ids are allocated separately by the approved ID provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SingleSkuMerchandiseIdentity {
    pub spu_no: String,
    pub sku_no: String,
}

#[async_trait]
pub trait SingleSkuMerchandiseRepositoryPort: Send + Sync {
    async fn list_skus(
        &self,
        query: SingleSkuMerchandiseListQuery,
    ) -> Result<SingleSkuMerchandisePage, CommerceServiceError>;

    async fn create_single_sku(
        &self,
        command: CreateSingleSkuMerchandiseCommand,
    ) -> Result<SkuRecord, CommerceServiceError>;

    async fn update_single_sku(
        &self,
        command: UpdateSingleSkuMerchandiseCommand,
    ) -> Result<SkuRecord, CommerceServiceError>;
}

/// Derives stable business numbers from the owner scope and idempotency key.
/// This helper never allocates database primary ids.
pub fn stable_identity(
    tenant_id: &str,
    organization_id: &str,
    fulfillment_type: &str,
    idempotency_key: &str,
) -> SingleSkuMerchandiseIdentity {
    let material = format!(
        "{tenant_id}\n{organization_id}\n{}\n{idempotency_key}",
        fulfillment_type.trim().to_ascii_lowercase()
    );
    let digest = sha256_hash(material.as_bytes());
    let token = &digest[..24];
    let prefix = slugify(fulfillment_type)
        .chars()
        .take(16)
        .collect::<String>();
    let prefix = if prefix.is_empty() {
        "item"
    } else {
        prefix.as_str()
    };
    SingleSkuMerchandiseIdentity {
        spu_no: format!("{prefix}-spu-{token}"),
        sku_no: format!("{prefix}-sku-{token}"),
    }
}

pub fn public_spec(record: &SkuRecord) -> Value {
    let Some(raw) = record.spec_json.as_deref() else {
        return Value::Object(Map::new());
    };
    let Ok(mut value) = serde_json::from_str::<Value>(raw) else {
        return Value::Object(Map::new());
    };
    let Some(object) = value.as_object_mut() else {
        return Value::Object(Map::new());
    };
    object.remove(SINGLE_SKU_MERCHANDISE_METADATA_KEY);
    value
}

pub fn description(record: &SkuRecord) -> Option<String> {
    let raw = record.spec_json.as_deref()?;
    let value = serde_json::from_str::<Value>(raw).ok()?;
    value
        .get(SINGLE_SKU_MERCHANDISE_METADATA_KEY)
        .and_then(|metadata| metadata.get(SINGLE_SKU_MERCHANDISE_DESCRIPTION_KEY))
        .and_then(Value::as_str)
        .map(str::to_owned)
}

pub fn normalize_public_spec(value: Value) -> Result<Value, CommerceServiceError> {
    let object = value
        .as_object()
        .ok_or_else(|| CommerceServiceError::validation("spec must be a JSON object"))?;
    if object.contains_key(SINGLE_SKU_MERCHANDISE_METADATA_KEY) {
        return Err(CommerceServiceError::validation(
            "spec must not contain reserved _sdkwork metadata",
        ));
    }
    Ok(value)
}

fn normalize_currency_code(value: &str) -> Result<String, CommerceServiceError> {
    let value = required_text("currency_code", value)?.to_ascii_uppercase();
    if value.len() != 3
        || !value
            .chars()
            .all(|character| character.is_ascii_uppercase())
    {
        return Err(CommerceServiceError::validation(
            "currency_code must be a three-letter ISO code",
        ));
    }
    Ok(value)
}

fn normalize_status(value: &str) -> Result<String, CommerceServiceError> {
    let value = required_text("status", value)?.to_ascii_lowercase();
    if matches!(value.as_str(), "draft" | "active" | "inactive") {
        Ok(value)
    } else {
        Err(CommerceServiceError::validation(
            "status must be draft, active, or inactive",
        ))
    }
}

fn parse_optional_money(
    field: &str,
    value: Option<&str>,
) -> Result<Option<CommerceMoney>, CommerceServiceError> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| {
            CommerceMoney::new(value)
                .map_err(|error| CommerceServiceError::validation(format!("{field}: {error}")))
        })
        .transpose()
}

fn parse_nullable_text_patch(
    field: &str,
    value: NullablePatch<&str>,
) -> Result<NullablePatch<String>, CommerceServiceError> {
    value
        .map(|value| value.map(|value| required_text(field, value)).transpose())
        .transpose()
}

fn parse_nullable_money_patch(
    field: &str,
    value: NullablePatch<&str>,
) -> Result<NullablePatch<CommerceMoney>, CommerceServiceError> {
    value
        .map(|value| {
            value
                .map(|value| {
                    CommerceMoney::new(value.trim()).map_err(|error| {
                        CommerceServiceError::validation(format!("{field}: {error}"))
                    })
                })
                .transpose()
        })
        .transpose()
}

fn required_text(field: &str, value: &str) -> Result<String, CommerceServiceError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(CommerceServiceError::validation(format!(
            "{field} must not be blank"
        )));
    }
    Ok(value.to_owned())
}

fn optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_stable_and_scoped() {
        let first = stable_identity("tenant-a", "org-a", "notary", "request-1");
        assert_eq!(
            first,
            stable_identity("tenant-a", "org-a", "notary", "request-1")
        );
        assert_ne!(
            first.sku_no,
            stable_identity("tenant-b", "org-a", "notary", "request-1").sku_no
        );
    }

    #[test]
    fn create_command_persists_managed_metadata_inside_spec() {
        let command = CreateSingleSkuMerchandiseCommand::new(
            "tenant",
            "org",
            "notary",
            "notary",
            "Remote notarization",
            Some("Description"),
            "100",
            None,
            "CNY",
            "active",
            serde_json::json!({"durationDays": 3}),
            "idem-1",
        )
        .expect("command");
        let persisted = command.persisted_spec().expect("valid command");
        assert_eq!(persisted["durationDays"], 3);
        assert_eq!(persisted["_sdkwork"]["description"], "Description");
        assert_eq!(
            public_spec(&SkuRecord {
                id: "sku".to_owned(),
                tenant_id: "tenant".to_owned(),
                organization_id: Some("org".to_owned()),
                spu_id: "spu".to_owned(),
                sku_no: "sku-no".to_owned(),
                name: "name".to_owned(),
                title: "title".to_owned(),
                price_amount: "100".to_owned(),
                original_price_amount: None,
                currency_code: "CNY".to_owned(),
                fulfillment_type: "notary".to_owned(),
                inventory_tracking: "untracked".to_owned(),
                status: "active".to_owned(),
                published_at: None,
                spec_json: Some(persisted.to_string()),
                created_at: "".to_owned(),
                updated_at: "".to_owned(),
            }),
            serde_json::json!({"durationDays": 3})
        );
    }

    #[test]
    fn spec_rejects_reserved_managed_metadata() {
        let result = CreateSingleSkuMerchandiseCommand::new(
            "tenant",
            "org",
            "notary",
            "notary",
            "Title",
            None,
            "100",
            None,
            "CNY",
            "active",
            serde_json::json!({"_sdkwork": {"description": "spoofed"}}),
            "request-1",
        );

        assert!(result.is_err());
    }

    #[test]
    fn update_command_preserves_nullable_patch_states() {
        let omitted = UpdateSingleSkuMerchandiseCommand::new(
            "tenant", "org", "sku", "notary", None, None, None, None, None, None, None, None,
        )
        .expect("omitted patch fields");
        assert_eq!(omitted.description, None);
        assert_eq!(omitted.original_price_amount, None);

        let cleared = UpdateSingleSkuMerchandiseCommand::new(
            "tenant",
            "org",
            "sku",
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
        .expect("clear patch fields");
        assert_eq!(cleared.description, Some(None));
        assert_eq!(cleared.original_price_amount, Some(None));

        let set = UpdateSingleSkuMerchandiseCommand::new(
            "tenant",
            "org",
            "sku",
            "notary",
            None,
            None,
            Some(Some(" Description ")),
            None,
            Some(Some("120")),
            None,
            None,
            None,
        )
        .expect("set patch fields");
        assert_eq!(set.description, Some(Some("Description".to_owned())));
        assert_eq!(
            set.original_price_amount
                .as_ref()
                .and_then(|value| value.as_ref())
                .map(|value| value.as_str()),
            Some("120")
        );

        assert!(UpdateSingleSkuMerchandiseCommand::new(
            "tenant",
            "org",
            "sku",
            "notary",
            None,
            None,
            Some(Some(" ")),
            None,
            None,
            None,
            None,
            None,
        )
        .is_err());
    }

    #[test]
    fn list_query_caps_page_size_and_offset() {
        assert!(
            SingleSkuMerchandiseListQuery::new("tenant", None, "notary", None, None, 201, 0)
                .is_err()
        );
        assert!(
            SingleSkuMerchandiseListQuery::new("tenant", None, "notary", None, None, 20, -1)
                .is_err()
        );
        assert!(SingleSkuMerchandiseListQuery::new(
            "tenant",
            None,
            "notary",
            None,
            None,
            20,
            i64::MAX,
        )
        .is_err());
    }
}
