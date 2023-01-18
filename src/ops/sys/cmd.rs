use crate::helpers;
use crate::{state, state::Permissions};

use engine::op;
use macros::function_path;
use shell::cmd;

#[op]
fn cmd_exec(cmd: String) -> String {
    state::error!(Permissions::allow_cmd(), state::error_cmd(function_path!()));
    cmd!(helpers::string_to_static_str(cmd)).stdout_utf8().unwrap()
}

#[op]
async fn cmd_spawn(cmd: String) -> String {
    state::error!(Permissions::allow_cmd(), state::error_cmd(function_path!()));
    cmd!(helpers::string_to_static_str(cmd)).stdout_utf8().unwrap()
}
