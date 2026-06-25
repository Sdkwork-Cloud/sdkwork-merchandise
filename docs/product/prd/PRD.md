# Merchandise PRD

Status: active
Owner: SDKWork maintainers
Application: merchandise
Updated: 2026-06-24
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## Document Map

- Platform split alignment (commerce T0): `../sdkwork-commerce/docs/architecture/tech/TECH-2026-06-24-commerce-capability-repo-split-alignment.md`

## 1. Background And Problem

Product master data (SPU/SKU, catalog admin) must be owned by a dedicated merchandise capability rather than a monolithic commerce crate.

This repository is a **T1 commerce capability building block**. `sdkwork-commerce` remains the T0 composition layer (gateway, IAM wrappers, composed SDK). This repository owns domain logic, persistence, and HTTP route builders for the **merchandise** capability.

## 2. Target Users

Merchant catalog administrators, commerce operators, and integrators publishing or maintaining product master data.

## 3. Goals And Non-Goals

### Goals

- Own merchandise catalog SQL, domain commands/queries, and backend admin catalog HTTP routers.
- Provide backend admin catalog surfaces consumed by commerce T0 with IAM wrappers.
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
- Shop routes in commerce consume merchandise catalog store traits for product coupling.
- OpenAPI and SDK generation include catalog operations through commerce composed surfaces.

## 6. Success Metrics

- Catalog routes pass commerce api-server integration tests via thin IAM wrappers.
- No local `sdkwork-commerce-catalog-service` duplicate in commerce workspace.

## 7. Phases

- Phase 1 (complete): SQL + backend admin catalog routes owned in sdkwork-merchandise.
- Phase 3 (complete): browse/open app routes owned by `sdkwork-catalog`.

## 8. Linked Requirements

- Commerce capability split alignment: `../sdkwork-commerce/docs/architecture/tech/TECH-2026-06-24-commerce-capability-repo-split-alignment.md`
- Component contract: `specs/component.spec.json` (when present)
- Machine contracts: local `specs/`, future `apis/`, and generated `sdks/`

## 9. Open Questions


