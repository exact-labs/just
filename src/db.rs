use deno_core::op;
use std::collections::HashMap;

#[op]
pub fn op_db_init(db_name: String) {
    sqlite::open(db_name).unwrap();
}

#[op]
pub fn op_db_create(db_name: String, table: String, keys: String) {
    let connection = sqlite::open(db_name).unwrap();
    connection
        .execute(format!("CREATE TABLE if not exists {table} ({keys})",))
        .unwrap();
}

#[op]
pub fn op_db_insert(db_name: String, table: String, keys: String, value: String) {
    let connection = sqlite::open(db_name).unwrap();
    connection
        .execute(format!("INSERT INTO {table} ({keys}) VALUES({value})",))
        .unwrap();
}

#[op]
pub fn op_db_query(db_name: String, table: String, query: String) {
    let connection = sqlite::open(db_name).unwrap();

    println!(
        "{:#?}",
        connection
            .prepare(format!("SELECT * from {table} {query}"))
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap())
            .collect::<Vec<_>>()
    )
}

#[op]
pub fn op_db_delete(db_name: String, table: String, query: String) {
    let connection = sqlite::open(db_name).unwrap();
    connection
        .execute(format!("DELETE FROM {table} WHERE {query}"))
        .unwrap();
}

#[op]
pub fn op_db_exec(db_name: String, query: String) {
    let connection = sqlite::open(db_name).unwrap();
    connection.execute(format!("{query}")).unwrap();
}
