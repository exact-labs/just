use crate::helpers;
use engine::op;
use shell::cmd;

#[op]
fn cmd_exec(cmd: String) -> String {
    return cmd!(helpers::string_to_static_str(cmd)).stdout_utf8().unwrap();
}

#[op]
async fn cmd_spawn(cmd: String) -> String {
    return cmd!(helpers::string_to_static_str(cmd)).stdout_utf8().unwrap();
}
