# Merchandise Technical Architecture

Status: active development  
Owner: SDKWork maintainers  
Updated: 2026-07-11  
Specs: `ARCHITECTURE_DECISION_SPEC.md`, `RUST_CODE_SPEC.md`, `API_SPEC.md`,
`WEB_FRAMEWORK_SPEC.md`, `DATABASE_SPEC.md`, `DATABASE_FRAMEWORK_SPEC.md`

## 1. Architecture Overview

`sdkwork-merchandise` is the commerce merchandise capability owner. It is
organized as a contract-first Rust service, a SQLx repository adapter, HTTP
route crates, and runtime gateway composition. The service and repository are
reusable building blocks; the gateway owns transport and IAM composition.

```text
application/admin or vertical domain
        |
        v
SingleSkuMerchandiseService
        |
        v
SingleSkuMerchandiseRepositoryPort
        |
        v
SqlxSingleSkuMerchandiseRepository
        |                 |
        |                 +-> injected SDKWork Snowflake IdGenerator
        v
existing commerce_product_spu/sku tables
```

The single-SKU boundary models one sellable service as one SPU and one SKU,
stores caller-visible fields in the existing SKU record, and reserves the
`_sdkwork` section of `spec_json` for owner metadata. The deterministic hash is
used only for `spu_no` and `sku_no`; runtime `id` values come from the approved
Snowflake provider. The boundary does not add tables or schema assets.

## 2. Technology Choices

- Rust domain and service contracts (`RUST_CODE_SPEC.md`)
- SQLx SQLite/PostgreSQL repository implementations
- `sdkwork-database` pool and lifecycle integration (`DATABASE_FRAMEWORK_SPEC.md`)
- Axum route crates integrated through `sdkwork-web-framework`
- Generated SDKs and SDKWork response envelopes for HTTP consumers

## 3. System Boundaries And Modules

| Layer | Owner | Responsibility |
| --- | --- | --- |
| Service contract | `sdkwork-merchandise-service` | Validation, typed commands/queries, repository port |
| Persistence adapter | `sdkwork-merchandise-repository-sqlx` | Tenant-scoped SQL, transactions, Snowflake ids, idempotency replay, bounded listing |
| App routes | `sdkwork-routes-merchandise-app-api` | App HTTP contract and SDK authority |
| Backend routes | `sdkwork-routes-merchandise-backend-api` | Operator/admin HTTP contract and SDK authority |
| Runtime composition | `sdkwork-merchandise-standalone-gateway` | Pool, IAM, route, and readiness wiring |
| Database lifecycle | approved database host/framework | Existing schema, migrations, drift, and health |

Vertical domains consume the service/repository boundary and reuse order and
payment owners. They do not own or write merchandise tables.

## 4. Directory And Package Layout

- `crates/sdkwork-merchandise-service/`
- `crates/sdkwork-merchandise-repository-sqlx/`
- `crates/sdkwork-routes-merchandise-app-api/`
- `crates/sdkwork-routes-merchandise-backend-api/`
- `crates/sdkwork-merchandise-database-host/`
- `crates/sdkwork-merchandise-service-host/`
- `crates/sdkwork-merchandise-standalone-gateway/`
- `crates/sdkwork-merchandise-gateway-assembly/`
- `apps/sdkwork-merchandise-pc/`

Each authored crate owns its local `specs/component.spec.json` contract.

## 5. API, SDK, And Data Ownership

- App API prefix: `/app/v3/api/catalog/products`
- Backend API prefix: `/backend/v3/api/catalog/products`
- Table authority: existing `commerce_product_spu` and
  `commerce_product_sku` records supplied by the database lifecycle owner
- New owner boundary: no DDL, migration, seed, or table-registry changes
- Public list contract: tenant/org/fulfillment/status/q filters with page size
  1..200 and SQL `LIMIT/OFFSET`
- Public write contract: typed validation, injected Snowflake primary ids,
  deterministic SPU/SKU business numbers, atomic transaction, and conflict on
  idempotency payload mismatch

HTTP handlers use `sdkwork-web-framework` response mapping; consumers use
scoped composed SDK packages rather than generated transport internals.

## 6. Security, Privacy, And Observability

- Tenant and organization predicates are applied before merchandise reads or
  writes.
- Fulfillment type scopes vertical ownership and update authority.
- Input JSON is object-only; `_sdkwork` metadata is owner-managed and rejected
  from caller payloads.
- SQL uses bound parameters; no user input is concatenated into statements.
- Errors map to typed service errors and must not expose raw SQL details at HTTP
  boundaries.
- Runtime hosts provide pool health, readiness, tracing, and audit integration.

## 7. Deployment And Runtime Topology

The repository supports embedded and standalone composition. Production pool
creation and migrations are performed by the approved database lifecycle host;
the repository adapter receives a configured `DatabasePool` and approved
`IdGenerator`. The host owns Snowflake node allocation and must retain any node
lease for the repository lifetime. The capability is still in pre-release
implementation and has no production deployment claim.

## 8. Architecture Decision Index

- Reusable merchandise owner boundary: documented in this architecture and the
  service/repository component specs.
- Existing database reuse: no new schema artifacts; see `DATABASE_SPEC.md` and
  `DATABASE_FRAMEWORK_SPEC.md`.

## 9. Verification

```bash
cargo check -p sdkwork-merchandise-service -p sdkwork-merchandise-repository-sqlx
cargo test -p sdkwork-merchandise-service -p sdkwork-merchandise-repository-sqlx
pnpm verify
```
