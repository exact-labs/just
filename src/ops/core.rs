use crate::helpers;
use crate::project;

use anyhow::Error;
use colored::Colorize;
use duration_string::DurationString;
use engine::{op, v8, OpDecl};
use macros::{function_name, ternary};
use nanoid::nanoid;
use std::io::{stdout, Write};
use std::{env, thread};
use v_htmlescape::escape;

pub fn init() -> Vec<OpDecl> {
    vec![
        setup::decl(),
        sleep::decl(),
        print::decl(),
        options::decl(),
        log_info::decl(),
        to_bytes::decl(),
        random_id::decl(),
        from_bytes::decl(),
        log_stderr::decl(),
        log_stdout::decl(),
        hex_encode::decl(),
        hex_decode::decl(),
        random_uuid::decl(),
        get_package::decl(),
        base64_encode::decl(),
        base64_decode::decl(),
        escape_string::decl(),
        runtime_memory::decl(),
    ]
}

#[op]
fn options() -> String {
    return crate::runtime::BootstrapOptions::as_json(&crate::runtime::BootstrapOptions::default());
}

#[op]
fn setup() {
    state::get::sys(function_name!());
    state::get::write(function_name!());
    crate::cli::setup();
}

#[op]
fn random_id(len: usize) -> String {
    return nanoid!(len);
}

#[op]
pub fn random_uuid() -> Result<String, Error> {
    Ok(uuid::Uuid::new_v4().to_string())
}

#[op]
fn escape_string(text: String) -> Result<String, Error> {
    Ok(escape(&text).to_string())
}

#[op]
fn to_bytes(string: String) -> Result<Vec<u8>, Error> {
    Ok(string.into_bytes())
}

#[op]
fn from_bytes(bytes: Vec<u8>) -> Result<String, Error> {
    Ok(String::from_utf8(bytes)?)
}

#[op]
fn hex_encode(string: String) -> Result<String, Error> {
    Ok(hex::encode(string))
}

#[op]
fn base64_encode(string: String) -> Result<String, Error> {
    Ok(base64::encode(string))
}

#[op]
fn hex_decode(string: String) -> Result<String, Error> {
    Ok(String::from_utf8(hex::decode(string)?).unwrap())
}

#[op]
fn base64_decode(string: String) -> Result<String, Error> {
    Ok(String::from_utf8(base64::decode(string)?).unwrap())
}

#[op]
fn get_package(package: String, version: String) -> String {
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
fn print(msg: String) -> Result<(), Error> {
    stdout().write_all(msg.as_bytes())?;
    stdout().flush().unwrap();
    Ok(())
}

#[op]
fn log_stdout(msg: String) -> Result<(), Error> {
    print!("{}\n", msg);
    Ok(())
}

#[op]
fn log_stderr(msg: String) -> Result<(), Error> {
    eprint!("{}\n", format!("{}", msg).red());
    Ok(())
}

#[op]
fn log_info(msg: String) -> Result<(), Error> {
    print!("{}\n", format!("{}", msg).cyan());
    Ok(())
}

#[op]
fn sleep(ms: String) -> Result<(), Error> {
    thread::sleep(DurationString::from_string(ms).unwrap().into());
    Ok(())
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct MemoryUsage {
    rss: usize,
    heap_total: usize,
    heap_used: usize,
    external: usize,
}

#[op(v8)]
fn runtime_memory(scope: &mut v8::HandleScope) -> MemoryUsage {
    let mut s = v8::HeapStatistics::default();
    scope.get_heap_statistics(&mut s);
    MemoryUsage {
        rss: helpers::rss(),
        heap_total: s.total_heap_size(),
        heap_used: s.used_heap_size(),
        external: s.external_memory(),
    }
}
