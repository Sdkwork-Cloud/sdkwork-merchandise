# sdkwork-merchandise-repository-sqlx

- Domain: `commerce`
- Capability: `merchandise`
- Package type: Rust SQLx repository crate
- Status: stable

This crate is the SQL persistence adapter for merchandise-owned data. It
implements service ports with the approved `sdkwork-database-sqlx`
`DatabasePool` and supports SQLite and PostgreSQL without owning pool
construction or schema lifecycle.

## Public API

- `SqlxSingleSkuMerchandiseRepository::new(DatabasePool, Arc<dyn IdGenerator>)`
  implements `SingleSkuMerchandiseRepositoryPort` with an injected approved ID
  provider.
- `SqlxSingleSkuMerchandiseRepository::with_snowflake_node_id` is a convenience
  constructor for hosts that already own a configured Snowflake node id.
- `SqliteCommerceCatalogStore` and `PostgresCommerceCatalogStore` expose the
  existing catalog repository surfaces.

The single-SKU repository performs tenant- and organization-scoped bounded
listing, atomic one-SPU/one-SKU creation, deterministic idempotency business
numbers, Snowflake primary ids, replay verification, and coordinated SPU/SKU
updates. Nullable PATCH fields bind presence separately from value so omission
preserves state while explicit null clears `description` or
`original_price_amount`.

## Required SDK Surface

None. The repository receives a database pool through native Rust composition
and does not call HTTP APIs.

## Configuration

The host must construct `DatabasePool` through `sdkwork-database` and inject an
approved `sdkwork-database-id` generator using its runtime profile. This crate
does not read environment variables, choose a production Snowflake node, or
create production pools directly.

## Deployment Profile And Runtime Target Behavior

SQLite and PostgreSQL use equivalent typed repository operations. SQL is
parameter-bound, listing is performed with store-level `LIMIT` and `OFFSET`,
and create/update operations use transactions. Database migrations and schema
registration remain owned by the database lifecycle layer.

## Security

Every single-SKU query and write includes tenant scope. Organization scope is
required for writes and supported as an explicit filter for tenant-wide admin
listing. Fulfillment type is part of the owner boundary so one consumer domain
cannot mutate another domain's SKU.

## Extension Points

Add new persistence behavior by implementing an existing merchandise service
port. Do not expose table-level CRUD to consumer domains, construct ad hoc
database pools, or add schema definitions to this crate.

## Verification

```bash
cargo test -p sdkwork-merchandise-repository-sqlx
```

## Owner And Status

The machine-readable component contract is
`specs/component.spec.json`. SDKWork global standards remain authoritative.
