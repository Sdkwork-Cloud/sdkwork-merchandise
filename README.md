# sdkwork-merchandise

SDKWork commerce **merchandise** capability building-block repository (domain `commerce`).

- Standards: `../sdkwork-specs/README.md`
- Composition consumer: `../sdkwork-clawrouter/vendor/sdkwork-commerce` (archived transitional platform snapshot)
- Domain service: `crates/sdkwork-commerce-merchandise-service/`
- Repository SQL: `crates/sdkwork-commerce-merchandise-repository-sqlx/`
- PC app: `apps/sdkwork-merchandise-pc/`
- HTTP API server: `crates/sdkwork-merchandise-api-server/`

## Quick start

```bash
pnpm verify
cargo test --workspace
```

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

## Application Roots

- [apps directory index](apps/README.md)
