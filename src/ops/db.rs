use engine::op;
use std::collections::HashMap;

#[op]
pub fn op_db_init(db_name: String) {
    sqlite::open(db_name).unwrap();
}

#[op]
pub fn op_db_create(db_name: String, table: String, keys: String) {
    let connection = sqlite::open(db_name).unwrap();
    connection.execute(format!("CREATE TABLE if not exists {table} ({keys})",)).unwrap();
}

#[op]
pub fn op_db_insert(db_name: String, table: String, keys: String, value: String) {
    let connection = sqlite::open(db_name).unwrap();
    connection.execute(format!("INSERT INTO {table} ({keys}) VALUES({value})",)).unwrap();
}

#[op]
pub fn op_db_query(db_name: String, table: String, query: String) -> String {
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
pub fn op_db_delete(db_name: String, table: String, query: String) {
    let connection = sqlite::open(db_name).unwrap();
    connection.execute(format!("DELETE FROM {table} {query}")).unwrap();
}

#[op]
pub fn op_db_exec(db_name: String, query: String) {
    let connection = sqlite::open(db_name).unwrap();
    connection.execute(format!("{query}")).unwrap();
}
