use crate::helpers;
use crate::{state, state::Permissions};

use anyhow::Error;
use engine::op;
use macros::function_path;
use std::collections::HashMap;
use std::str::from_utf8;

#[op]
fn kv_get(path: String, key: String) -> Result<String, Error> {
    state::error!(Permissions::allow_read(), state::error_read(function_path!()));
    let db = sled::open(&path)?;
    let value = db.get(&key)?.unwrap();
    let utf8 = from_utf8(&value)?;

    Ok(String::from(utf8))
}

#[op]
fn kv_set(path: String, key: String, value: String) -> Result<(), Error> {
    state::error!(Permissions::allow_write(), state::error_write(function_path!()));
    let db = sled::open(&path)?;
    db.insert(&key, sled::IVec::from(helpers::string_to_static_str(value)))?;
    db.flush()?;

    Ok(())
}

#[op]
fn kv_remove(path: String, key: String) -> Result<(), Error> {
    state::error!(Permissions::allow_write(), state::error_write(function_path!()));
    let db = sled::open(&path)?;
    db.remove(&key)?;
    db.flush()?;

    Ok(())
}

#[op]
fn kv_range(path: String, start: String, end: String) -> Result<HashMap<String, String>, Error> {
    state::error!(Permissions::allow_read(), state::error_read(function_path!()));
    let db = sled::open(&path)?;
    let mut store: HashMap<String, String> = HashMap::new();

    for result in db.range(start..end) {
        let (key, value) = result.clone()?;
        store.insert(String::from(from_utf8(&key)?), String::from(from_utf8(&value)?));
    }

    println!("{:?}", store);
    Ok(store)
}
