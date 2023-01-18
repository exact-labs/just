use crate::helpers;

use engine::op;
use macros::function_name;
use shell::cmd;

pub fn init() -> Vec<engine::OpDecl> {
    vec![external_function::decl()]
}

#[op]
fn external_function(name: String, args: String) -> String {
    state::get::cmd(function_name!());
    state::get::sys(function_name!());
    return match cmd!(helpers::string_to_static_str(format!("{}/.just/bin/lib_ext.bin {name} {args}", home::home_dir().unwrap().display()))).stdout_utf8() {
        Ok(output) => output,
        Err(err) => format!("{:?}", err),
    };
}
