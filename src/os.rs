use deno_core::op;
use std::process::Command;

#[op]
pub fn op_release() -> String {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("failed to execute process");

    return format!("{}", String::from_utf8_lossy(&output.stdout));
}

#[op]
pub fn op_platform() -> String {
    let output = Command::new("uname")
        .arg("-s")
        .output()
        .expect("failed to execute process");

    return format!("{}", String::from_utf8_lossy(&output.stdout));
}
