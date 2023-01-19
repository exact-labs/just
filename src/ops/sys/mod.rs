pub mod cmd;
pub mod env;
pub mod os;

pub fn init() -> Vec<engine::OpDecl> {
    vec![
        cmd::cmd_exec::decl(),
        cmd::cmd_spawn::decl(),
        env::env_local::decl(),
        env::env_get::decl(),
        env::env_set::decl(),
        os::os_release::decl(),
        os::os_platform::decl(),
        os::os_machine::decl(),
        os::os_hostname::decl(),
        os::os_homedir::decl(),
        os::os_cpus::decl(),
        os::os_uptime::decl(),
        os::os_memory::decl(),
        os::os_loadavg::decl(),
        os::os_dirname::decl(),
        os::os_exit::decl(),
    ]
}
