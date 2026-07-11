# sdkwork-merchandise-service component specs

This directory is the local contract index for the merchandise service crate.
The machine-readable authority is [component.spec.json](./component.spec.json);
global SDKWork standards remain authoritative.

## Component

| Field | Value |
| --- | --- |
| Name | `sdkwork-merchandise-service` |
| Type | `rust-crate` |
| Root | `crates/sdkwork-merchandise-service` |
| Domain | `commerce` |
| Capability | `merchandise` |
| Layer | `backend-service` |
| Status | `stable` |

## Public Contract

- Service port: `SingleSkuMerchandiseRepositoryPort`.
- Service facade: `SingleSkuMerchandiseService`.
- Nullable update contract: `NullablePatch<T>` distinguishes omitted, set, and
  explicit-clear states for `description` and `original_price_amount`.
- Package export: `.`.
- No HTTP route or generated SDK is owned by this crate.

## Canonical Specs

- [COMPONENT_SPEC.md](../../../../sdkwork-specs/COMPONENT_SPEC.md)
- [MODULE_SPEC.md](../../../../sdkwork-specs/MODULE_SPEC.md)
- [DOMAIN_SPEC.md](../../../../sdkwork-specs/DOMAIN_SPEC.md)
- [API_SPEC.md](../../../../sdkwork-specs/API_SPEC.md)
- [PAGINATION_SPEC.md](../../../../sdkwork-specs/PAGINATION_SPEC.md)
- [CODE_STYLE_SPEC.md](../../../../sdkwork-specs/CODE_STYLE_SPEC.md)
- [NAMING_SPEC.md](../../../../sdkwork-specs/NAMING_SPEC.md)
- [RUST_CODE_SPEC.md](../../../../sdkwork-specs/RUST_CODE_SPEC.md)
- [SECURITY_SPEC.md](../../../../sdkwork-specs/SECURITY_SPEC.md)
- [TEST_SPEC.md](../../../../sdkwork-specs/TEST_SPEC.md)

## Verification

```bash
cargo test -p sdkwork-merchandise-service
```
