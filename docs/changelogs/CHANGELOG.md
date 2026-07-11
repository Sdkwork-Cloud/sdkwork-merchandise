# Merchandise Changelog

## Unreleased - 2026-07-11

- Added the reusable `SingleSkuMerchandiseRepositoryPort` and service facade
  for one-SPU/one-SKU merchandise operations.
- Added SQLite/PostgreSQL SQLx persistence with tenant/org/fulfillment scoping,
  bounded store-level pagination, atomic writes, injected Snowflake primary
  ids, deterministic idempotency business numbers, and SKU `spec_json`/status
  updates.
- Documented the owner boundary and explicitly kept database schema lifecycle
  outside the new capability; no DDL or migration was added.
- Corrected merchandise module contracts and the catalog test fixtures to use
  the current smallest-unit `CommerceMoney` format.
