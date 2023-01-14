use crate::fn_name;
use crate::state;
use crate::state::Permissions;
use crate::state_err;

use engine::op;
use std::collections::HashMap;

#[op]
fn sqlite_init(db_name: String) {
    state_err!(Permissions::allow_read(), state::error_read(fn_name!()));
    sqlite::open(db_name).unwrap();
}

#[op]
fn sqlite_create(db_name: String, table: String, keys: String) {
    state_err!(Permissions::allow_write(), state::error_write(fn_name!()));
    let connection = sqlite::open(db_name).unwrap();
    connection.execute(format!("CREATE TABLE if not exists {table} ({keys})",)).unwrap();
}

#[op]
fn sqlite_insert(db_name: String, table: String, keys: String, value: String) {
    state_err!(Permissions::allow_write(), state::error_write(fn_name!()));
    let connection = sqlite::open(db_name).unwrap();
    connection.execute(format!("INSERT INTO {table} ({keys}) VALUES({value})",)).unwrap();
}

#[op]
fn sqlite_query(db_name: String, table: String, query: String) -> String {
    state_err!(Permissions::allow_read(), state::error_read(fn_name!()));
    let connection = sqlite::open(db_name).unwrap();
    let mut rows: Vec<String> = Vec::new();

    connection
        .iterate(format!("SELECT * from {table} {query}"), |pairs| {
            let mut column = HashMap::<&str, &str>::new();
            for &(name, value) in pairs.iter() {
                column.insert(name, value.unwrap());
            }
            rows.push(format!("{:?}", column));
            true
        })
        .unwrap();

    return format!("{:?}", rows);
}

#[op]
fn sqlite_delete(db_name: String, table: String, query: String) {
    state_err!(Permissions::allow_write(), state::error_write(fn_name!()));
    let connection = sqlite::open(db_name).unwrap();
    connection.execute(format!("DELETE FROM {table} {query}")).unwrap();
}

#[op]
fn sqlite_exec(db_name: String, query: String) {
    state_err!(Permissions::allow_read(), state::error_read(fn_name!()));
    let connection = sqlite::open(db_name).unwrap();
    connection.execute(format!("{query}")).unwrap();
}
