use sdkwork_contract_service::{CommerceMoney, CommerceSurfaceProfile};
use sdkwork_merchandise_service::{
    catalog_service_contract, BuyerAddressDraft, CartItemDraft, CatalogPortRequirement,
    CatalogRepositoryCommand, FulfillmentType, InventoryTrackingMode, ProductAttributeDraft,
    ProductCategoryDraft, ProductSkuDraft, ProductSpuDraft, ProductStatus, ProductType,
};

#[test]
fn catalog_domain_contract_uses_standard_spu_sku_terms_without_legacy_product_table_debt() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let checked_sources = [
        "Cargo.toml",
        "src/lib.rs",
        "src/domain/mod.rs",
        "src/commands/mod.rs",
        "src/queries/mod.rs",
        "src/ports/mod.rs",
        "src/service/mod.rs",
    ];
    let banned_fragments = [
        concat!("commerce_", "product\""),
        concat!("commerce_", "sku\""),
        concat!("product", "_id"),
        concat!("product", "_no"),
        "SalesStatus",
        "DeliveryMode",
        "sales_status",
        "delivery_mode",
        "parent_category_id",
        "sort_weight",
    ];

    let mut violations = Vec::new();
    for relative_path in checked_sources {
        let contents = std::fs::read_to_string(crate_root.join(relative_path))
            .unwrap_or_else(|error| panic!("failed to read {relative_path}: {error}"));
        for banned in banned_fragments {
            if contents.contains(banned) {
                violations.push(format!("{relative_path} contains `{banned}`"));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "catalog domain must use explicit SPU/SKU terminology without legacy product table fragments:\n{}",
        violations.join("\n")
    );
}

#[test]
fn validates_product_spu_for_physical_virtual_membership_and_points_products() {
    let spu = ProductSpuDraft::new(
        "100001",
        "0",
        "spu-membership-pro",
        "Pro membership",
        ProductType::Membership,
        Some("cat-membership"),
        ProductStatus::Active,
        vec![CommerceSurfaceProfile::App, CommerceSurfaceProfile::Console],
    )
    .unwrap();

    assert_eq!(spu.spu_no, "spu-membership-pro");
    assert_eq!(spu.status.as_storage_str(), "active");
    assert_eq!(spu.product_type.as_storage_str(), "membership");
    assert_eq!(ProductType::Physical.as_storage_str(), "physical");
    assert_eq!(ProductType::Virtual.as_storage_str(), "virtual");
    assert_eq!(
        ProductType::PointsRecharge.as_storage_str(),
        "points_recharge"
    );
    assert!(spu.visible_surfaces.contains(&CommerceSurfaceProfile::App));
    assert!(ProductSpuDraft::new(
        "100001",
        "0",
        "",
        "No code",
        ProductType::Physical,
        None,
        ProductStatus::Active,
        vec![CommerceSurfaceProfile::App],
    )
    .is_err());
}

#[test]
fn validates_product_sku_fulfillment_and_inventory_modes() {
    let sku = ProductSkuDraft::new(
        "100001",
        "0",
        "sku-membership-month-pro",
        "spu-membership-pro",
        "Monthly Pro membership",
        CommerceMoney::new("69.90").unwrap(),
        Some(CommerceMoney::new("129.00").unwrap()),
        "CNY",
        FulfillmentType::MembershipActivation,
        InventoryTrackingMode::Untracked,
    )
    .unwrap();

    assert_eq!(sku.spu_id, "spu-membership-pro");
    assert_eq!(
        sku.fulfillment_type.as_storage_str(),
        "membership_activation"
    );
    assert_eq!(sku.inventory_tracking.as_storage_str(), "untracked");
    assert_eq!(
        FulfillmentType::PhysicalShipment.as_storage_str(),
        "physical_shipment"
    );
    assert_eq!(
        FulfillmentType::VirtualDelivery.as_storage_str(),
        "virtual_delivery"
    );
    assert_eq!(
        FulfillmentType::PointsCredit.as_storage_str(),
        "points_credit"
    );
    assert_eq!(InventoryTrackingMode::Tracked.as_storage_str(), "tracked");
    assert!(ProductSkuDraft::new(
        "100001",
        "0",
        "sku-bad",
        "spu-physical",
        "Bad physical SKU",
        CommerceMoney::new("1.00").unwrap(),
        None,
        "",
        FulfillmentType::PhysicalShipment,
        InventoryTrackingMode::Tracked,
    )
    .is_err());
}

#[test]
fn validates_categories_attributes_cart_items_and_buyer_addresses() {
    let category =
        ProductCategoryDraft::new("100001", "0", "cat-membership", None, "Membership", 100)
            .unwrap();
    let attribute = ProductAttributeDraft::new(
        "100001",
        "0",
        "attr-duration",
        "Duration",
        vec!["30 days".to_owned(), "365 days".to_owned()],
    )
    .unwrap();
    let cart_item = CartItemDraft::new("100001", "1", "sku-membership-month-pro", 2).unwrap();
    let address = BuyerAddressDraft::new(
        "100001",
        "1",
        "addr-1",
        "Alice",
        "13800000000",
        "CN",
        "Shanghai",
        "Pudong",
        "Lane 1",
        true,
    )
    .unwrap();

    assert_eq!(category.category_no, "cat-membership");
    assert_eq!(category.parent_id, None);
    assert_eq!(category.sort_order, 100);
    assert_eq!(attribute.values.len(), 2);
    assert_eq!(cart_item.quantity, 2);
    assert!(address.is_default);
    assert!(ProductAttributeDraft::new("100001", "0", "attr-empty", "Empty", Vec::new()).is_err());
    assert!(CartItemDraft::new("100001", "1", "sku-1", 0).is_err());
}

#[test]
fn catalog_repository_contract_exposes_required_commands() {
    assert_eq!(
        CatalogPortRequirement::standard_commands(),
        vec![
            CatalogRepositoryCommand::CreateCategory,
            CatalogRepositoryCommand::CreateAttribute,
            CatalogRepositoryCommand::CreateSpu,
            CatalogRepositoryCommand::CreateSku,
            CatalogRepositoryCommand::AddCartItem,
            CatalogRepositoryCommand::RemoveCartItem,
            CatalogRepositoryCommand::UpsertBuyerAddress,
        ],
    );
}

#[test]
fn catalog_service_contract_exposes_domain_operations() {
    let contract = catalog_service_contract();

    assert_eq!(contract.domain, "catalog");
    assert_eq!(contract.service_name, "commerce.catalog");
    assert!(contract.validate().is_ok());
    for query in [
        "catalog.categories.list",
        "catalog.attributes.list",
        "catalog.products.list",
        "catalog.products.retrieve",
        "catalog.categoryAttributes.list",
        "catalog.spus.list",
        "catalog.spus.retrieve",
        "catalog.skus.list",
        "catalog.skus.retrieve",
        "catalog.skus.prices.retrieve",
        "cart.current.retrieve",
        "addresses.list",
    ] {
        assert!(
            contract.read_queries.contains(&query),
            "catalog contract must expose read query {query}",
        );
    }
    for command in [
        "catalog.categories.create",
        "catalog.attributes.create",
        "catalog.products.create",
        "catalog.products.update",
        "catalog.products.delete",
        "catalog.spus.create",
        "catalog.spus.update",
        "catalog.spus.publish",
        "catalog.spus.archive",
        "catalog.skus.create",
        "catalog.skus.update",
        "catalog.skus.delete",
        "catalog.categorySeeds.create",
        "catalog.categoryAttributes.create",
        "catalog.categoryAttributes.update",
        "catalog.categoryAttributes.delete",
        "cart.items.create",
        "cart.items.delete",
        "addresses.create",
        "addresses.update",
        "addresses.delete",
        "addresses.defaultSelection.create",
    ] {
        assert!(
            contract.write_commands.contains(&command),
            "catalog contract must expose write command {command}",
        );
    }
}
