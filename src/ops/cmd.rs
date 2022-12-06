use crate::helpers;
use deno_core::op;
use shell::cmd;

#[op]
pub fn op_exec(cmd: String) -> String {
    return cmd!(helpers::string_to_static_str(cmd)).stdout_utf8().unwrap();
}

#[op]
pub async fn op_spawn(cmd: String) -> String {
    return cmd!(helpers::string_to_static_str(cmd)).stdout_utf8().unwrap();
}
