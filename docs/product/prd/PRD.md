# Merchandise PRD

Status: active development  
Owner: SDKWork maintainers  
Application: `sdkwork-merchandise`  
Updated: 2026-07-11  
Specs: `REQUIREMENTS_SPEC.md`, `DOCUMENTATION_SPEC.md`, `DOMAIN_SPEC.md`

## 1. Background And Problem

Commerce applications need one authoritative owner for sellable SPU/SKU master
data. Domain features such as notary services must be able to model a service
as merchandise and reuse the common order and payment systems without creating
parallel product tables or bespoke persistence.

## 2. Target Users

Commerce administrators, vertical-domain operators, backend integrators, and
client applications that publish or select sellable merchandise.

## 3. Goals And Non-Goals

### Goals

- Own SPU/SKU validation, lifecycle status, fulfillment classification, and
  tenant/organization isolation.
- Provide a reusable one-SPU/one-SKU boundary for service-like merchandise,
  including Snowflake primary ids, deterministic idempotency business numbers,
  and bounded management listing.
- Let vertical capabilities reuse existing commerce order and payment
  ownership without duplicating merchandise or database definitions.
- Keep HTTP routes, SDK generation, and persistence aligned with SDKWork
  contracts.

### Non-Goals

- Owning order, payment, IAM, or Drive lifecycle.
- Introducing new product tables, migrations, seeds, or schema registries for a
  vertical capability.
- Hand-written HTTP clients or direct consumer writes to commerce tables.

## 4. Scope

- SPU/SKU catalog master data and backend administration.
- Backend catalog route surface: `/backend/v3/api/catalog/*`, contributed to
  `sdkwork-shop-backend-api` and generated in `sdkwork-shop-backend-sdk`.
- Merchandise service ports and SQLx repository adapters for SQLite and
  PostgreSQL.
- Reusable single-SKU merchandise operations:
  tenant/org/fulfillment/status filtering, bounded offset pagination,
  `spec_json` public metadata, injected Snowflake IDs, deterministic SPU/SKU
  numbers, atomic create, idempotent replay detection, and scoped update.

The database lifecycle and table authority remain external to this repository's
new boundary. No DDL or migration is added by the single-SKU capability.

## 5. User Scenarios

- An operator creates a notary service as one SPU with one SKU and a stable
  idempotency key.
- An operator searches one tenant and organization one page at a time, filtered
  by fulfillment type, status, or text.
- A vertical domain updates title, price, status, or public specification
  through the merchandise owner and then creates an order through the order
  owner.
- A retry with the same idempotency key returns the same SKU when the payload
  matches and fails closed when it differs.

## 6. Success Metrics

- All owner-boundary tests pass on the service and SQLx repository packages.
- No consumer domain performs direct SPU/SKU writes.
- List paths enforce store-level page bounds and tenant predicates.
- No new database definitions are required for vertical merchandise adoption.

## 7. Delivery Phase

- Current phase: implementation and contract alignment.
- Production release: not yet declared.
- Completion gate: route, Shop backend SDK authority, security, database,
  documentation, and topology checks pass in the owning repositories.

## 8. Linked Requirements

- Component contracts: `crates/*/specs/component.spec.json`
- Technical architecture: [TECH_ARCHITECTURE.md](../../architecture/tech/TECH_ARCHITECTURE.md)
- Change evidence: [CHANGELOG.md](../../changelogs/CHANGELOG.md)
- Canonical standards: `../../../../sdkwork-specs/`

## 9. Open Questions

Shop owns the backend API authority and SDK family. Merchandise owns the
catalog capability implementation and contributes its route module under the
canonical `commerce.catalog.read` and `commerce.catalog.manage` permissions.
