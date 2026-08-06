import io, re, os

def read(p):
    return io.open(p, 'r', encoding='utf-8').read().replace('\r\n', '\n')

def write(p, c):
    io.open(p, 'w', encoding='utf-8', newline='\n').write(c)

def strip_impl(content, marker):
    lines = content.split('\n')
    out = []
    i = 0
    removed = 0
    while i < len(lines):
        stripped = lines[i].strip()
        if stripped.startswith('impl ') and marker in stripped and stripped.endswith('{'):
            j = i
            brace = 0
            while j < len(lines):
                brace += lines[j].count('{') - lines[j].count('}')
                if brace <= 0:
                    break
                j += 1
            i = j + 1
            removed += 1
            continue
        out.append(lines[i])
        i += 1
    return '\n'.join(out), removed

def strip_fn(content, name):
    lines = content.split('\n')
    out = []
    i = 0
    removed = False
    while i < len(lines):
        if re.search(rf'\bfn {name}\b', lines[i].strip()):
            j = i
            found_open = False
            brace = 0
            while j < len(lines):
                if not found_open:
                    if '{' in lines[j]:
                        found_open = True
                        brace = lines[j].count('{') - lines[j].count('}')
                    j += 1
                    continue
                brace += lines[j].count('{') - lines[j].count('}')
                if brace <= 0:
                    break
                j += 1
            i = j + 1
            removed = True
            continue
        out.append(lines[i])
        i += 1
    assert removed, name
    return '\n'.join(out), removed

def remove_tests_mod(content):
    lines = content.split('\n')
    out = []
    i = 0
    changed = False
    while i < len(lines):
        if lines[i].strip() == '#[cfg(test)]' and i + 1 < len(lines) and re.match(r'mod tests \{', lines[i+1].strip()):
            j = i + 1
            brace = 0
            found_open = False
            while j < len(lines):
                if not found_open:
                    if '{' in lines[j]:
                        found_open = True
                        brace = lines[j].count('{') - lines[j].count('}')
                    j += 1
                    continue
                brace += lines[j].count('{') - lines[j].count('}')
                if brace <= 0:
                    break
                j += 1
            block = '\n'.join(lines[i:j+1])
            if re.search(r'[sS]qlite', block):
                i = j + 1
                changed = True
                continue
        out.append(lines[i])
        i += 1
    return '\n'.join(out), changed

# ============ repository-sqlx ============
for f in ['sqlite_catalog.rs', 'single_sku_merchandise/sqlite.rs', 'single_sku_merchandise/tests.rs']:
    p = f'crates/sdkwork-merchandise-repository-sqlx/src/{f}'
    if os.path.exists(p):
        os.remove(p)
        print('deleted', f)

p = 'crates/sdkwork-merchandise-repository-sqlx/src/lib.rs'
c = read(p)
c = c.replace('pub mod sqlite_catalog;\n', '')
c = c.replace('pub use sqlite_catalog::SqliteCommerceCatalogStore;\n', '')
write(p, c)
print('repo lib cleaned')

p = 'crates/sdkwork-merchandise-repository-sqlx/src/single_sku_merchandise.rs'
c = read(p)
c = c.replace('mod sqlite;\n', '')
c = re.sub(r'#\[cfg\(test\)\]\nmod tests;\n', '', c)
write(p, c)
print('single_sku mod cleaned')

# ============ web-support ============
p = 'crates/sdkwork-merchandise-web-support/src/catalog_store.rs'
c = read(p)
c, _ = strip_impl(c, 'for SqliteCommerceCatalogStore')
c = c.replace('    PostgresCommerceCatalogStore, SqliteCommerceCatalogStore,', '    PostgresCommerceCatalogStore,')
write(p, c)
print('catalog_store cleaned')

p = 'crates/sdkwork-merchandise-web-support/src/backend_catalog_router.rs'
c = read(p)
c = c.replace('    PostgresCommerceCatalogStore, SqliteCommerceCatalogStore,', '    PostgresCommerceCatalogStore,')
m = re.search(r'pub fn backend_catalog_router_with_sqlite_pool\(pool: SqlitePool\) -> Router \{\n    build_backend_catalog_router\(Arc::new\(SqliteCommerceCatalogStore::new\(pool\)\)\)\n\}\n\n', c)
if m:
    c = c[:m.start()] + c[m.end():]
c = c.replace('use sqlx::{PgPool, SqlitePool};', 'use sqlx::PgPool;')
write(p, c)
print('backend_catalog_router cleaned')

p = 'crates/sdkwork-merchandise-web-support/src/lib.rs'
c = read(p)
c = c.replace('    backend_catalog_router_with_postgres_pool, backend_catalog_router_with_sqlite_pool,', '    backend_catalog_router_with_postgres_pool,')
write(p, c)
print('web-support lib cleaned')

# ============ routes-backend ============
p = 'crates/sdkwork-routes-merchandise-backend-api/src/routes.rs'
c = read(p)
old = '''pub fn build_merchandise_backend_router(host: Arc<MerchandiseServiceHost>) -> Router {
    match host.database_pool() {
        DatabasePool::Postgres(pool, _) => backend_catalog_router_with_postgres_pool(pool.clone()),
        DatabasePool::Sqlite(pool, _) => backend_catalog_router_with_sqlite_pool(pool.clone()),
    }
}'''
new = '''pub fn build_merchandise_backend_router(host: Arc<MerchandiseServiceHost>) -> Router {
    // 服务端权威持久化仅支持 PostgreSQL（DATABASE_SPEC：authoritative-server）
    let DatabasePool::Postgres(pool, _) = host.database_pool() else {
        panic!("merchandise backend router requires a PostgreSQL database pool");
    };
    backend_catalog_router_with_postgres_pool(pool.clone())
}'''
assert old in c, 'routes arm'
c = c.replace(old, new)
c = c.replace('''use sdkwork_merchandise_web_support::{
    backend_catalog_router_with_postgres_pool, backend_catalog_router_with_sqlite_pool,
};''', '''use sdkwork_merchandise_web_support::backend_catalog_router_with_postgres_pool;''')
write(p, c)
print('routes.rs cleaned')

# ============ root features ============
p = 'Cargo.toml'
c = read(p)
c = c.replace('features = ["runtime-tokio", "postgres", "sqlite", "uuid", "chrono"]',
              'features = ["runtime-tokio", "postgres", "json", "uuid", "chrono"]')
c = c.replace('features = ["postgres", "sqlite"]', 'features = ["postgres"]')
write(p, c)
print('root features done')
