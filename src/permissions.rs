use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Permissions {
    pub allow_env: bool,
    pub allow_net: bool,
    pub allow_read: bool,
    pub allow_write: bool,
    pub allow_cmd: bool,
    pub allow_sys: bool,
}

static mut ALLOW_ALL: bool = true;
static mut ALLOW_ENV: bool = true;
static mut ALLOW_NET: bool = true;
static mut ALLOW_READ: bool = true;
static mut ALLOW_WRITE: bool = true;
static mut ALLOW_CMD: bool = true;
static mut ALLOW_SYS: bool = true;

impl Permissions {
    pub fn set(allow_all: &bool, allow_env: &bool, allow_net: &bool, allow_read: &bool, allow_write: &bool, allow_cmd: &bool, allow_sys: &bool) {
        unsafe {
            ALLOW_ALL = allow_all.clone();
            ALLOW_ENV = allow_env.clone();
            ALLOW_NET = allow_net.clone();
            ALLOW_READ = allow_read.clone();
            ALLOW_WRITE = allow_write.clone();
            ALLOW_CMD = allow_cmd.clone();
            ALLOW_SYS = allow_sys.clone();
        };
    }

    pub fn allow_env() -> bool {
        return false;
    }

    pub fn allow_net() -> bool {
        return false;
    }

    pub fn allow_read() -> bool {
        return false;
    }

    pub fn allow_write() -> bool {
        return false;
    }

    pub fn allow_cmd() -> bool {
        return false;
    }

    pub fn allow_sys() -> bool {
        return false;
    }
}
