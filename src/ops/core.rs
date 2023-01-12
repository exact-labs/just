use crate::helpers;
use crate::project;
use crate::ternary;
use anyhow::Error;
use colored::Colorize;
use duration_string::DurationString;
use engine::{op, OpDecl};
use nanoid::nanoid;
use std::io::{stdout, Write};
use std::{env, thread};
use v_htmlescape::escape;

pub fn init() -> Vec<OpDecl> {
    vec![
        setup::decl(),
        sleep::decl(),
        print::decl(),
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
        runtime_version::decl(),
    ]
}

#[op]
fn setup() {
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
fn runtime_version() -> String {
    return format!("{}", env!("CARGO_PKG_VERSION"));
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
    let decoded = String::from_utf8(bytes)?;
    Ok(decoded)
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
    let decoded = hex::decode(string)?;
    Ok(String::from_utf8(decoded).unwrap())
}

#[op]
fn base64_decode(string: String) -> Result<String, Error> {
    let decoded = base64::decode(string)?;
    Ok(String::from_utf8(decoded).unwrap())
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
