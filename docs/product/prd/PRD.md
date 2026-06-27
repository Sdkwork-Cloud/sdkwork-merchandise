# Merchandise PRD

Status: active
Owner: SDKWork maintainers
Application: merchandise
Updated: 2026-06-24
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## Document Map

- Commerce repository dissolution: `../sdkwork-specs/MIGRATION_SPEC.md` §8

## 1. Background And Problem

Product master data (SPU/SKU, catalog admin) must be owned by a dedicated merchandise capability rather than a monolithic commerce crate.

This repository is a **T1 commerce capability building block**. The `sdkwork-commerce` monolith has been dissolved; this repository is self-contained with its own domain logic, persistence, HTTP route builders, API server, and IAM middleware for the **merchandise** capability.

## 2. Target Users

Merchant catalog administrators, commerce operators, and integrators publishing or maintaining product master data.

## 3. Goals And Non-Goals

### Goals

- Own merchandise catalog SQL, domain commands/queries, and backend admin catalog HTTP routers.
- Provide backend admin catalog surfaces consumed by the T1 `*-standalone-gateway` with IAM wrappers.
- Keep table prefix and API naming aligned with commerce domain standards.

### Non-Goals

- Public browse/open catalog routes (owned by `sdkwork-catalog`).
- Shop deposit or order lifecycle.
- Hand-written HTTP bypassing generated SDK contracts.

## 4. Scope

- SPU/SKU catalog master data and backend admin mutations.
- Backend catalog list/create/update routes.
- Merchandise repository SQLx implementations and shared catalog store trait.

Primary API prefixes:

- App: `null`
- Backend: `/backend/v3/api/catalog`

Migration status: **complete**.

## 5. User Scenarios

- A catalog admin creates an SPU with SKUs and publishes it to the tenant catalog.
- Shop routes in sibling T1 repos consume merchandise catalog store traits for product coupling.
- OpenAPI and SDK generation include catalog operations through per-T1 SDK families.

## 6. Success Metrics

- Catalog routes pass T1 standalone-gateway integration tests via IAM wrappers.
- No local `sdkwork-commerce-catalog-service` duplicate in any workspace.

## 7. Phases

- Phase 1 (complete): SQL + backend admin catalog routes owned in sdkwork-merchandise.
- Phase 3 (complete): browse/open app routes owned by `sdkwork-catalog`.

## 8. Linked Requirements

- Commerce repository dissolution: `../sdkwork-specs/MIGRATION_SPEC.md` §8
- Component contract: `specs/component.spec.json` (when present)
- Machine contracts: local `specs/`, future `apis/`, and generated `sdks/`

## 9. Open Questions


