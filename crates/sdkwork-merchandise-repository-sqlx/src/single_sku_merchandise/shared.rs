use sdkwork_contract_service::CommerceServiceError;
use sdkwork_database_id::IdGenerator;
use sdkwork_merchandise_service::{
    normalize_public_spec, CreateSingleSkuMerchandiseCommand, SingleSkuMerchandiseListQuery,
    SingleSkuMerchandisePage, SkuRecord, UpdateSingleSkuMerchandiseCommand,
    SINGLE_SKU_MERCHANDISE_DESCRIPTION_KEY, SINGLE_SKU_MERCHANDISE_METADATA_KEY,
    SINGLE_SKU_MERCHANDISE_POLICY_KEY, SINGLE_SKU_MERCHANDISE_PRODUCT_TYPE_KEY,
};
use serde_json::{Map, Value};

#[derive(Debug)]
pub(super) struct SpuReplayRecord {
    pub(super) id: String,
    pub(super) spu_no: String,
    pub(super) title: String,
    pub(super) status: String,
}

pub(super) fn finalize_page(
    query: &SingleSkuMerchandiseListQuery,
    items: &mut Vec<SkuRecord>,
) -> SingleSkuMerchandisePage {
    let has_more = items.len() as i64 > query.page_size;
    items.truncate(query.page_size as usize);
    SingleSkuMerchandisePage {
        items: std::mem::take(items),
        has_more,
        next_offset: has_more.then_some(query.offset.saturating_add(query.page_size)),
    }
}

pub(super) fn ensure_single_sku_count(count: i64) -> Result<(), CommerceServiceError> {
    if count == 1 {
        Ok(())
    } else {
        Err(CommerceServiceError::conflict(
            "single-SKU merchandise SPU must reference exactly one SKU",
        ))
    }
}

pub(super) fn next_entity_id(
    id_generator: &dyn IdGenerator,
    resource: &str,
) -> Result<String, CommerceServiceError> {
    let id = id_generator.next_id().map_err(|error| {
        CommerceServiceError::storage(format!("generate {resource} id: {error}"))
    })?;
    if id.parse::<i64>().is_ok_and(|value| value > 0) {
        Ok(id)
    } else {
        Err(CommerceServiceError::storage(format!(
            "generate {resource} id: provider returned a non-positive snowflake id"
        )))
    }
}

pub(super) fn verify_create_replay(
    record: &SkuRecord,
    command: &CreateSingleSkuMerchandiseCommand,
    sku_no: &str,
    persisted_spec: &Value,
) -> Result<(), CommerceServiceError> {
    let actual_spec = record
        .spec_json
        .as_deref()
        .and_then(|value| serde_json::from_str::<Value>(value).ok())
        .unwrap_or_else(|| Value::Object(Map::new()));
    let matches = record.sku_no == sku_no
        && record.title == command.title
        && record.price_amount == command.price_amount.as_str()
        && record.original_price_amount.as_deref()
            == command
                .original_price_amount
                .as_ref()
                .map(|value| value.as_str())
        && record
            .currency_code
            .eq_ignore_ascii_case(&command.currency_code)
        && record
            .fulfillment_type
            .eq_ignore_ascii_case(&command.fulfillment_type)
        && record.status.eq_ignore_ascii_case(&command.status)
        && actual_spec == *persisted_spec;
    if matches {
        Ok(())
    } else {
        Err(CommerceServiceError::conflict(
            "idempotency key was already used with a different merchandise payload",
        ))
    }
}

pub(super) fn verify_spu_replay(
    record: &SpuReplayRecord,
    command: &CreateSingleSkuMerchandiseCommand,
    spu_no: &str,
) -> Result<(), CommerceServiceError> {
    if record.spu_no == spu_no
        && record.title == command.title
        && record.status.eq_ignore_ascii_case(&command.status)
    {
        Ok(())
    } else {
        Err(CommerceServiceError::conflict(
            "idempotency key was already used with a different merchandise SPU payload",
        ))
    }
}

pub(super) fn ensure_sku_spu_link(
    sku: &SkuRecord,
    spu: &SpuReplayRecord,
) -> Result<(), CommerceServiceError> {
    if sku.spu_id == spu.id {
        Ok(())
    } else {
        Err(CommerceServiceError::conflict(
            "idempotent merchandise SKU references a different SPU",
        ))
    }
}

pub(super) fn ensure_fulfillment_type(
    record: &SkuRecord,
    fulfillment_type: &str,
) -> Result<(), CommerceServiceError> {
    if record
        .fulfillment_type
        .eq_ignore_ascii_case(fulfillment_type)
    {
        Ok(())
    } else {
        Err(CommerceServiceError::not_found(
            "merchandise SKU was not found for fulfillment type",
        ))
    }
}

pub(super) fn updated_spec_json(
    current: &SkuRecord,
    command: &UpdateSingleSkuMerchandiseCommand,
) -> Result<String, CommerceServiceError> {
    let current_value = current
        .spec_json
        .as_deref()
        .and_then(|value| serde_json::from_str::<Value>(value).ok())
        .filter(Value::is_object)
        .unwrap_or_else(|| Value::Object(Map::new()));
    let replacement = command
        .spec
        .clone()
        .map(normalize_public_spec)
        .transpose()?;
    let mut next = replacement.unwrap_or_else(|| current_value.clone());
    let next_object = next
        .as_object_mut()
        .ok_or_else(|| CommerceServiceError::validation("spec must be a JSON object"))?;
    let current_metadata = current_value
        .get(SINGLE_SKU_MERCHANDISE_METADATA_KEY)
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let metadata = next_object
        .entry(SINGLE_SKU_MERCHANDISE_METADATA_KEY)
        .or_insert_with(|| Value::Object(current_metadata));
    let metadata = metadata.as_object_mut().ok_or_else(|| {
        CommerceServiceError::validation("reserved merchandise metadata must be an object")
    })?;
    metadata.insert(
        SINGLE_SKU_MERCHANDISE_POLICY_KEY.to_owned(),
        Value::String("one_spu_one_sku".to_owned()),
    );
    if let Some(product_type) = &command.product_type {
        metadata.insert(
            SINGLE_SKU_MERCHANDISE_PRODUCT_TYPE_KEY.to_owned(),
            Value::String(product_type.clone()),
        );
    }
    match &command.description {
        None => {}
        Some(Some(description)) => {
            metadata.insert(
                SINGLE_SKU_MERCHANDISE_DESCRIPTION_KEY.to_owned(),
                Value::String(description.clone()),
            );
        }
        Some(None) => {
            metadata.remove(SINGLE_SKU_MERCHANDISE_DESCRIPTION_KEY);
        }
    }
    serde_json::to_string(&next)
        .map_err(|error| CommerceServiceError::validation(format!("serialize spec: {error}")))
}

pub(super) fn store_error(context: &str, error: sqlx::Error) -> CommerceServiceError {
    CommerceServiceError::storage(format!("{context}: {error}"))
}
