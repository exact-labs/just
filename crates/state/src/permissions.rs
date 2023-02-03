use lazy_static::lazy_static;
use macros::ternary;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(clap::Args, Debug, Clone)]
pub struct Builder {
    #[arg(short = 'A', long, default_value_t = false, help = "Allow all permissions")]
    pub allow_all: bool,
    #[arg(short = 'E', long, default_value_t = false, help = "Allow environment access", conflicts_with = "allow_all")]
    pub allow_env: bool,
    #[arg(short = 'N', long, default_value_t = false, help = "Allow network access", conflicts_with = "allow_all")]
    pub allow_net: bool,
    #[arg(short = 'R', long, default_value_t = false, help = "Allow file system read access", conflicts_with = "allow_all")]
    pub allow_read: bool,
    #[arg(short = 'W', long, default_value_t = false, help = "Allow file system write access", conflicts_with = "allow_all")]
    pub allow_write: bool,
    #[arg(short = 'C', long, default_value_t = false, help = "Allow running subprocesses", conflicts_with = "allow_all")]
    pub allow_cmd: bool,
    #[arg(short = 'S', long, default_value_t = false, help = "Allow access to system info", conflicts_with = "allow_all")]
    pub allow_sys: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Permissions {
    pub allow_env: bool,
    pub allow_net: bool,
    pub allow_read: bool,
    pub allow_write: bool,
    pub allow_cmd: bool,
    pub allow_sys: bool,
}

lazy_static! {
    static ref ALLOW_ALL: AtomicBool = AtomicBool::new(false);
    static ref ALLOW_ENV: AtomicBool = AtomicBool::new(false);
    static ref ALLOW_NET: AtomicBool = AtomicBool::new(false);
    static ref ALLOW_READ: AtomicBool = AtomicBool::new(false);
    static ref ALLOW_WRITE: AtomicBool = AtomicBool::new(false);
    static ref ALLOW_CMD: AtomicBool = AtomicBool::new(false);
    static ref ALLOW_SYS: AtomicBool = AtomicBool::new(false);
}

pub fn set(cli: &Builder) {
    ALLOW_ALL.store(cli.allow_all.clone(), Ordering::Relaxed);
    ALLOW_ENV.store(cli.allow_env.clone(), Ordering::Relaxed);
    ALLOW_NET.store(cli.allow_net.clone(), Ordering::Relaxed);
    ALLOW_CMD.store(cli.allow_cmd.clone(), Ordering::Relaxed);
    ALLOW_SYS.store(cli.allow_sys.clone(), Ordering::Relaxed);
    ALLOW_READ.store(cli.allow_read.clone(), Ordering::Relaxed);
    ALLOW_WRITE.store(cli.allow_write.clone(), Ordering::Relaxed);
}

pub fn env() -> bool {
    ternary!(ALLOW_ALL.load(Ordering::Relaxed), true, ALLOW_ENV.load(Ordering::Relaxed))
}
pub fn net() -> bool {
    ternary!(ALLOW_ALL.load(Ordering::Relaxed), true, ALLOW_NET.load(Ordering::Relaxed))
}
pub fn cmd() -> bool {
    ternary!(ALLOW_ALL.load(Ordering::Relaxed), true, ALLOW_CMD.load(Ordering::Relaxed))
}
pub fn sys() -> bool {
    ternary!(ALLOW_ALL.load(Ordering::Relaxed), true, ALLOW_SYS.load(Ordering::Relaxed))
}
pub fn read() -> bool {
    ternary!(ALLOW_ALL.load(Ordering::Relaxed), true, ALLOW_READ.load(Ordering::Relaxed))
}
pub fn write() -> bool {
    ternary!(ALLOW_ALL.load(Ordering::Relaxed), true, ALLOW_WRITE.load(Ordering::Relaxed))
}
