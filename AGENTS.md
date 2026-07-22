# Repository Guidelines

Read `../sdkwork-specs/SOUL.md` before work in this repository. Global SDKWork
standards remain authoritative; this file narrows them for the Merchandise
capability and does not duplicate their normative bodies.

## Capability Identity

- Repository: `sdkwork-merchandise`
- Domain: `commerce`
- Capability: `merchandise`
- Database module: `merchandise`
- Database service code: `MERCHANDISE`
- Database table prefix: `commerce_`
- Runtime family: Rust/Axum with `sdkwork-web-framework`

## API And SDK Ownership

- This repository implements the Merchandise backend catalog route module at
  `/backend/v3/api/catalog/*`.
- The canonical authority is `sdkwork-shop-backend-api`; the generated family
  is `sdkwork-shop-backend-sdk`.
- The Shop assembly mounts `sdkwork-api-merchandise-assembly` in the same
  origin and generates the combined owner-only backend SDK.
- Do not create `sdkwork-merchandise-app-api`,
  `sdkwork-merchandise-backend-api`, `sdkwork-merchandise-app-sdk`, or a
  second backend SDK authority.
- `sdkwork-merchandise-web-support` owns shared HTTP DTO, response, mapping,
  and store-port support only; it does not own routes or an API surface.
- Do not hand-edit generated SDK output. Change the authored OpenAPI module or
  generator input and run the approved Shop sdkgen workflow.

## Implementation Rules

- Rust HTTP handlers use `sdkwork-web-framework` and `sdkwork-utils-rust`.
- Success bodies use `SdkWorkApiResponse` with numeric `code: 0`, `data`, and
  `traceId`; errors use `application/problem+json` `ProblemDetail`.
- Create returns `201`; delete returns `204` without a body; list responses use
  `data.items` and `data.pageInfo`.
- Shared database bootstrap uses one process pool and
  `bootstrap_merchandise_database`; do not introduce Shop-named compatibility
  aliases in Merchandise crates.
- Do not add database schema, DDL, or migration changes without explicit
  database-owner review.

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

## Verification

Run from this repository root:

```powershell
cargo metadata --no-deps --format-version 1
cargo fmt -- --check
cargo test --workspace
pnpm api:check
pnpm api:check:route-manifest
pnpm api:assembly:validate
pnpm check
pnpm verify
node ../sdkwork-specs/tools/check-api-operation-patterns.mjs --workspace .
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
node ../sdkwork-specs/tools/check-pagination.mjs --workspace .
```
