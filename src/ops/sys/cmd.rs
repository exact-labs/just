use crate::fn_name;
use crate::helpers;
use crate::state;
use crate::state::Permissions;
use crate::state_err;

use engine::op;
use shell::cmd;

#[op]
fn cmd_exec(cmd: String) -> String {
    state_err!(Permissions::allow_cmd(), state::error_cmd(fn_name!()));
    cmd!(helpers::string_to_static_str(cmd)).stdout_utf8().unwrap()
}

#[op]
async fn cmd_spawn(cmd: String) -> String {
    state_err!(Permissions::allow_cmd(), state::error_cmd(fn_name!()));
    cmd!(helpers::string_to_static_str(cmd)).stdout_utf8().unwrap()
}
