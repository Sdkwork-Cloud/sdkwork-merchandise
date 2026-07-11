# sdkwork-merchandise-service

- Domain: `commerce`
- Capability: `merchandise`
- Package type: Rust service crate
- Status: stable

This crate owns merchandise-domain validation and service ports for SPU/SKU
master data. It does not create database pools or perform HTTP transport.
Persistence is supplied through the repository port by the composition root.

## Public API

- `SingleSkuMerchandiseListQuery` for tenant-, organization-, fulfillment-,
  status-, and text-filtered bounded listing.
- `CreateSingleSkuMerchandiseCommand` for one-SPU/one-SKU creation with
  deterministic idempotency business numbers and a public JSON specification.
- `UpdateSingleSkuMerchandiseCommand` for scoped SKU updates, including
  nullable PATCH semantics, `spec_json` metadata, and lifecycle status.
- `SingleSkuMerchandiseRepositoryPort` and `SingleSkuMerchandiseService` as the
  owner-level integration boundary.
- `public_spec`, `description`, and `normalize_public_spec` for safe handling
  of the managed `_sdkwork` metadata envelope.

The package-level export is the supported integration entrypoint. Consumers
must not depend on internal module paths.

## Required SDK Surface

None. This is a domain/service crate and does not own an HTTP SDK.

## Configuration

No environment access is performed here. Tenants, organizations, page bounds,
and fulfillment type are supplied through typed commands and queries. Database
pool construction belongs to the approved `sdkwork-database` composition layer.

## Deployment Profile And Runtime Target Behavior

The crate runs embedded in a host or behind the merchandise gateways. It is
runtime-neutral and supports the host's SQLite or PostgreSQL repository
implementation through `SingleSkuMerchandiseRepositoryPort`.

## Security

All repository operations require tenant and organization scope. List queries
are bounded to at most 200 records per page, and writes validate money, status,
currency, JSON object shape, and reserved metadata ownership before persistence.
The `_sdkwork` metadata namespace is service-managed and cannot be supplied by
callers.

## Nullable PATCH Semantics

`description` and `original_price_amount` use `NullablePatch<T>` at the owner
boundary. `None` means the field was omitted and preserves persisted state,
`Some(Some(value))` sets a validated value, and `Some(None)` explicitly clears
the nullable field. Blank strings are not clear aliases; callers must use the
explicit null state.

## Extension Points

Implement `SingleSkuMerchandiseRepositoryPort` in an approved owner repository
and inject it into `SingleSkuMerchandiseService`. Keep SPU/SKU persistence and
transaction handling in the repository owner; do not duplicate table writes in
consumer domains.

## Verification

```bash
cargo test -p sdkwork-merchandise-service
```

## Owner And Status

The machine-readable component contract is
`specs/component.spec.json`. SDKWork global standards remain authoritative.
