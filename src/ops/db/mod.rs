pub mod kv;
pub mod sqlite;

pub fn init() -> Vec<engine::OpDecl> {
    vec![
        kv::kv_get::decl(),
        kv::kv_set::decl(),
        kv::kv_remove::decl(),
        kv::kv_range::decl(),
        sqlite::sqlite_init::decl(),
        sqlite::sqlite_create::decl(),
        sqlite::sqlite_insert::decl(),
        sqlite::sqlite_query::decl(),
        sqlite::sqlite_delete::decl(),
        sqlite::sqlite_exec::decl(),
    ]
}
