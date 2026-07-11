# sdkwork-merchandise-repository-sqlx component specs

This directory is the local contract index for the SQLx merchandise repository.
The machine-readable authority is [component.spec.json](./component.spec.json);
global SDKWork standards remain authoritative.

## Component

| Field | Value |
| --- | --- |
| Name | `sdkwork-merchandise-repository-sqlx` |
| Type | `rust-crate` |
| Root | `crates/sdkwork-merchandise-repository-sqlx` |
| Domain | `commerce` |
| Capability | `merchandise` |
| Layer | `backend-repository` |
| Status | `stable` |

## Public Contract

- Repository adapter: `SqlxSingleSkuMerchandiseRepository`.
- Required runtime dependencies: `sdkwork_database_sqlx::DatabasePool` and an
  approved `sdkwork_database_id::IdGenerator`.
- Implemented service port: `SingleSkuMerchandiseRepositoryPort`.
- Update persistence preserves `NullablePatch<T>` tri-state semantics for
  nullable description metadata and original price.
- No database schema, migration, HTTP route, or SDK authority is owned here.

## Canonical Specs

- [COMPONENT_SPEC.md](../../../../sdkwork-specs/COMPONENT_SPEC.md)
- [MODULE_SPEC.md](../../../../sdkwork-specs/MODULE_SPEC.md)
- [DOMAIN_SPEC.md](../../../../sdkwork-specs/DOMAIN_SPEC.md)
- [DATABASE_SPEC.md](../../../../sdkwork-specs/DATABASE_SPEC.md)
- [DATABASE_FRAMEWORK_SPEC.md](../../../../sdkwork-specs/DATABASE_FRAMEWORK_SPEC.md)
- [SUBJECT_ID_SPEC.md](../../../../sdkwork-specs/SUBJECT_ID_SPEC.md)
- [PAGINATION_SPEC.md](../../../../sdkwork-specs/PAGINATION_SPEC.md)
- [CODE_STYLE_SPEC.md](../../../../sdkwork-specs/CODE_STYLE_SPEC.md)
- [NAMING_SPEC.md](../../../../sdkwork-specs/NAMING_SPEC.md)
- [RUST_CODE_SPEC.md](../../../../sdkwork-specs/RUST_CODE_SPEC.md)
- [SECURITY_SPEC.md](../../../../sdkwork-specs/SECURITY_SPEC.md)
- [TEST_SPEC.md](../../../../sdkwork-specs/TEST_SPEC.md)

## Verification

```bash
cargo test -p sdkwork-merchandise-repository-sqlx
```
