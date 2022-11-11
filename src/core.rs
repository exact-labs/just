use colored::Colorize;
use deno_core::error::AnyError;
use deno_core::op;
use duration_string::DurationString;
use std::{env, thread};
use v_htmlescape::escape;

#[op]
pub fn op_escape(text: String) -> Result<String, AnyError> {
    Ok(escape(&text).to_string())
}

#[op]
pub fn op_packages_dir() -> String {
    let dir = env::current_dir().unwrap();
    return format!("{}/packages", dir.display());
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