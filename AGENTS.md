# Repository Guidelines

## SDKWORK Soul

Read `../sdkwork-specs/SOUL.md` before executing tasks in this root.

## SDKWORK Standards

- `../sdkwork-specs/README.md`
- `../sdkwork-specs/SOUL.md`
- `../sdkwork-specs/AGENTS_SPEC.md`
- `../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`
- `../sdkwork-specs/DATABASE_FRAMEWORK_SPEC.md`

## Application Identity

Application manifests live under `apps/*/sdkwork.app.config.json`. This repository root is the commerce merchandise capability workspace (`domain: commerce`, `capability: merchandise`).

## Project Rules

- Canonical domain: `commerce`; capability: `merchandise` (`DOMAIN_SPEC.md`).
- This repository is the **authoritative owner** of merchandise capability (SPU/SKU/product master data).
- Composition consumer: `../sdkwork-clawrouter/vendor/sdkwork-commerce` (archived transitional platform snapshot)
- Merchandise capability scope: SPU/SKU and product master data per DOMAIN_SPEC.md.
- Database table prefix: `commerce_` for merchandise-owned tables.
- App API prefix: `/app/v3/api/catalog/products`.
- Backend API prefix: `/backend/v3/api/catalog/products`.
- Rust HTTP runtimes integrate `sdkwork-web-framework`; database lifecycle uses `sdkwork-database`.
- TypeScript packages consume `@sdkwork/utils-typescript` for shared helpers — no local duplicates.
- `sdkwork-discovery` is deferred until RPC/cloud-split deployment exists.
- Generated SDK output under `sdks/**/generated/**` is generator-owned.

## Verification

```bash
pnpm verify
pnpm db:validate
```

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

