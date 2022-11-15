use deno_core::op;
use shell::cmd;

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[op]
pub fn op_exec(cmd: String) -> String {
    return cmd!(string_to_static_str(cmd)).stdout_utf8().unwrap();
}

#[op]
pub async fn op_spawn(cmd: String) -> String {
    return cmd!(string_to_static_str(cmd)).stdout_utf8().unwrap();
}
