use anyhow::Error;
use engine::op;
use macros::{crash, function_name};
use std::process;

#[op]
fn cmd_exec(name: String, args: Vec<String>, path: String) -> Result<String, Error> {
    state::get::cmd(function_name!());
    let output = process::Command::new(name).args(args).current_dir(path).output()?;

    if !output.status.success() {
        crash!("{}", String::from_utf8(output.stderr)?);
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[op]
async fn cmd_spawn(name: String, args: Vec<String>, path: String) -> Result<String, Error> {
    state::get::cmd(function_name!());
    let output = async_process::Command::new(name).args(args).current_dir(path).output().await?;

    if !output.status.success() {
        crash!("{}", String::from_utf8(output.stderr)?);
    }

    Ok(String::from_utf8(output.stdout)?)
}
