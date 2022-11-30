use colored::Colorize;
use deno_core::error::AnyError;
use deno_core::op;
use duration_string::DurationString;
use nanoid::nanoid;
use serde::Deserialize;
use std::fs;
use std::{env, thread};
use v_htmlescape::escape;

#[derive(Debug, Deserialize)]
struct Project {
    index: String,
}

fn read_index(dir: String) -> Project {
    let contents = fs::read_to_string(format!("{dir}/package.yml")).unwrap();
    let yaml_file: Result<Project, _> = serde_yaml::from_str(&contents);

    let parsed = match yaml_file {
        Ok(project) => project,
        Err(error) => {
            eprintln!("{}", format!("{} in package.yml", error).red());
            std::process::exit(1);
        }
    };

    return parsed;
}

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
pub fn op_package_dir(package: String) -> String {
    let dir = env::current_dir().unwrap();
    return format!("{}/packages/{package}/", dir.display());
}

#[op]
pub fn op_package_index(package: String) -> String {
    let dir = env::current_dir().unwrap();
    return read_index(format!("{}/packages/{package}/", dir.display())).index;
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
