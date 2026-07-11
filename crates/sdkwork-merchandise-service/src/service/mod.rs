use sdkwork_contract_service::CommerceServiceContract;

mod single_sku_merchandise;

pub use single_sku_merchandise::SingleSkuMerchandiseService;

pub fn catalog_service_contract() -> CommerceServiceContract {
    CommerceServiceContract::new(
        "catalog",
        "commerce.catalog",
        vec![
            "catalog.categories.create",
            "catalog.categories.update",
            "catalog.categories.delete",
            "catalog.attributes.create",
            "catalog.priceLists.create",
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
            "cart.items.update",
            "cart.items.delete",
            "addresses.create",
            "addresses.update",
            "addresses.delete",
            "addresses.defaultSelection.create",
        ],
        vec![
            "catalog.categories.list",
            "catalog.attributes.list",
            "catalog.priceLists.list",
            "catalog.categoryAttributes.list",
            "catalog.products.list",
            "catalog.products.retrieve",
            "catalog.spus.list",
            "catalog.spus.retrieve",
            "catalog.skus.list",
            "catalog.skus.retrieve",
            "catalog.skus.prices.retrieve",
            "cart.current.retrieve",
            "addresses.list",
        ],
        vec![
            crate::ports::CATALOG_REPOSITORY_PORT,
            crate::ports::CART_REPOSITORY_PORT,
            crate::ports::BUYER_ADDRESS_REPOSITORY_PORT,
            crate::ports::IDEMPOTENCY_REPOSITORY_PORT,
            crate::ports::SINGLE_SKU_MERCHANDISE_REPOSITORY_PORT,
        ],
        true,
    )
}
