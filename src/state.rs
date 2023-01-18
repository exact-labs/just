use colored::Colorize;
use lazy_static::lazy_static;
use macros::ternary;
use serde::{Deserialize, Serialize};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};

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

impl Permissions {
    pub fn set(allow_all: &bool, allow_env: &bool, allow_net: &bool, allow_read: &bool, allow_write: &bool, allow_cmd: &bool, allow_sys: &bool) {
        ALLOW_ALL.store(allow_all.clone(), Ordering::Relaxed);
        ALLOW_ENV.store(allow_env.clone(), Ordering::Relaxed);
        ALLOW_NET.store(allow_net.clone(), Ordering::Relaxed);
        ALLOW_CMD.store(allow_cmd.clone(), Ordering::Relaxed);
        ALLOW_SYS.store(allow_sys.clone(), Ordering::Relaxed);
        ALLOW_READ.store(allow_read.clone(), Ordering::Relaxed);
        ALLOW_WRITE.store(allow_write.clone(), Ordering::Relaxed);
    }

    pub fn allow_env() -> bool {
        ternary!(ALLOW_ALL.load(Ordering::Relaxed), true, ALLOW_ENV.load(Ordering::Relaxed))
    }
    pub fn allow_net() -> bool {
        ternary!(ALLOW_ALL.load(Ordering::Relaxed), true, ALLOW_NET.load(Ordering::Relaxed))
    }
    pub fn allow_cmd() -> bool {
        ternary!(ALLOW_ALL.load(Ordering::Relaxed), true, ALLOW_CMD.load(Ordering::Relaxed))
    }
    pub fn allow_sys() -> bool {
        ternary!(ALLOW_ALL.load(Ordering::Relaxed), true, ALLOW_SYS.load(Ordering::Relaxed))
    }
    pub fn allow_read() -> bool {
        ternary!(ALLOW_ALL.load(Ordering::Relaxed), true, ALLOW_READ.load(Ordering::Relaxed))
    }
    pub fn allow_write() -> bool {
        ternary!(ALLOW_ALL.load(Ordering::Relaxed), true, ALLOW_WRITE.load(Ordering::Relaxed))
    }
}

pub fn error_env(path: &str) {
    eprintln!(
        "{} {}",
        format!("({path})").bright_red(),
        "[ENV] Uncaught Permission Denied: to access system env, please run again with the --allow-env flag".red()
    );
    exit(1)
}
pub fn error_net(path: &str) {
    eprintln!(
        "{} {}",
        format!("({path})").bright_red(),
        "[NETWORK] Permission Denied: to access the internet, please run again with the --allow-net flag".red()
    );
    exit(1)
}
pub fn error_cmd(path: &str) {
    eprintln!(
        "{} {}",
        format!("({path})").bright_red(),
        "[SPAWN] Permission Denied: to run a subprocess, please run again with the --allow-cmd flag".red()
    );
    exit(1)
}
pub fn error_sys(path: &str) {
    eprintln!(
        "{} {}",
        format!("({path})").bright_red(),
        "[SYSTEM] Permission Denied: to allow system functions, please run again with the --allow-sys flag".red()
    );
    exit(1)
}
pub fn error_read(path: &str) {
    eprintln!(
        "{} {}",
        format!("({path})").bright_red(),
        "[READ] Permission Denied: to read a file, please run again with the --allow-read flag".red()
    );
    exit(1)
}
pub fn error_write(path: &str) {
    eprintln!(
        "{} {}",
        format!("({path})").bright_red(),
        "[WRITE] Permission Denied: to write a file, please run again with the --allow-write flag".red()
    );
    exit(1)
}

macro_rules! error {
    ($c:expr, $v:expr) => {
        if !$c {
            $v
        }
    };
}

pub(crate) use error;
