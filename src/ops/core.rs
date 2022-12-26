use crate::helpers;
use crate::project;
use crate::ternary;
use colored::Colorize;
use deno_core::error::AnyError;
use deno_core::op;
use duration_string::DurationString;
use nanoid::nanoid;
use std::{env, thread};
use v_htmlescape::escape;

#[op]
pub fn op_id(len: usize) -> String {
    return nanoid!(len);
}

#[op]
fn op_version() -> String {
    return format!("{}", env!("CARGO_PKG_VERSION"));
}

#[op]
pub fn op_escape(text: String) -> Result<String, AnyError> {
    Ok(escape(&text).to_string())
}

#[op]
pub fn op_get_package(package: String, version: String) -> String {
    let dir = env::current_dir().unwrap();
    let dependencies = project::package::read().dependencies;

    let mut version_buffer = dependencies[&package].split(',').map(|s| s.to_string()).collect::<Vec<String>>();
    version_buffer.sort_by(|a, b| b.cmp(a));

    let package_version = ternary!(version != "", &version, &version_buffer[0]);
    let package_index = helpers::read_index(dir.display(), &package, &package_version).info.index;
    // insert error handler ^

    return format!("{}/packages/{package}/{}/{package_index}", dir.display(), &package_version);
}

#[op]
pub fn op_stdout(msg: String) -> Result<(), AnyError> {
    print!("{}\n", msg);
    Ok(())
}

#[op]
pub fn op_stderr(msg: String) -> Result<(), AnyError> {
    eprint!("{}\n", format!("{}", msg).red());
    Ok(())
}

#[op]
pub fn op_info(msg: String) -> Result<(), AnyError> {
    print!("{}\n", format!("{}", msg).cyan());
    Ok(())
}

#[op]
pub fn op_sleep(ms: String) -> Result<(), AnyError> {
    thread::sleep(DurationString::from_string(ms).unwrap().into());
    Ok(())
}
