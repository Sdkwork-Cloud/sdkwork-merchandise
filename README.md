# sdkwork-merchandise

repository-kind: application

SDKWork Commerce merchandise capability repository (domain `commerce`). This
repository owns SPU/SKU master data, catalog services, SQLx adapters, and the
merchandise HTTP route assemblies. It is an active building block and is not
yet a production release.

- Standards: [sdkwork-specs](../sdkwork-specs/README.md)
- Domain service: [sdkwork-merchandise-service](crates/sdkwork-merchandise-service/)
- SQL repository: [sdkwork-merchandise-repository-sqlx](crates/sdkwork-merchandise-repository-sqlx/)
- Backend routes: [sdkwork-routes-merchandise-backend-api](crates/sdkwork-routes-merchandise-backend-api/)
- Shared HTTP support: [sdkwork-merchandise-web-support](crates/sdkwork-merchandise-web-support/)
- Runtime gateways: `crates/sdkwork-api-merchandise-standalone-gateway/` and
  `crates/sdkwork-api-merchandise-assembly/`
- PC application: [apps/sdkwork-merchandise-pc](apps/sdkwork-merchandise-pc/)

## Capability Ownership

The merchandise service owns product master-data validation and typed ports.
The SQLx repository owns SPU/SKU persistence and transactions. Vertical
capabilities select their merchandise through `fulfillment_type` and consume
the public service/repository boundary; they do not write merchandise tables
directly. Runtime primary ids come from an approved `sdkwork-database-id`
provider injected by the host, while deterministic SPU/SKU business numbers
provide idempotent replay lookup. Existing commerce database tables and the
approved database lifecycle remain the schema authority. This repository adds
no DDL or migrations for that boundary.

## API Surfaces

- Backend catalog routes: `/backend/v3/api/catalog/*`
- API authority: `sdkwork-shop-backend-api`
- Generated SDK family: `sdkwork-shop-backend-sdk`

The Merchandise route crate contributes 27 owner-only catalog operations to
the Shop backend authority. This repository does not publish a duplicate
Merchandise app API, open API, or SDK family. HTTP handlers and the generated
Shop backend SDK follow the SDKWork response-envelope, pagination,
authentication, and error standards.

## Quick Start

```bash
pnpm verify
cargo test --workspace
```

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)
- [docs/changelogs/CHANGELOG.md](docs/changelogs/CHANGELOG.md)

## Application Roots

- [apps directory index](apps/README.md)
