use crate::ternary;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
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
