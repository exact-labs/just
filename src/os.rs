use deno_core::op;
use std::{env, process};
use sysinfo::{System, SystemExt};

#[op]
pub fn op_env_get(var: String) -> String {
    return env::var(var).unwrap_or("none".to_string());
}

#[op]
pub fn op_env_set(key: String, var: String) {
    env::set_var(key, var);
}

#[op]
pub fn op_release() -> String {
    #[cfg(target_os = "linux")]
    {
        match std::fs::read_to_string("/proc/sys/kernel/osrelease") {
            Ok(mut s) => {
                s.pop();
                s
            }
            _ => String::from(""),
        }
    }
    #[cfg(target_vendor = "apple")]
    {
        let mut s = [0u8; 256];
        let mut mib = [libc::CTL_KERN, libc::KERN_OSRELEASE];
        let mut len = s.len();
        if unsafe {
            libc::sysctl(
                mib.as_mut_ptr(),
                mib.len() as _,
                s.as_mut_ptr() as _,
                &mut len,
                std::ptr::null_mut(),
                0,
            )
        } == -1
        {
            return String::from("Unknown");
        }

        return String::from_utf8_lossy(&s[..len - 1]).to_string();
    }
    #[cfg(target_family = "windows")]
    {
        use ntapi::ntrtl::RtlGetVersion;
        use winapi::shared::ntdef::NT_SUCCESS;
        use winapi::um::winnt::RTL_OSVERSIONINFOEXW;

        let mut version_info = std::mem::MaybeUninit::<RTL_OSVERSIONINFOEXW>::uninit();
        unsafe {
            (*version_info.as_mut_ptr()).dwOSVersionInfoSize =
                std::mem::size_of::<RTL_OSVERSIONINFOEXW>() as u32;
        }
        if !NT_SUCCESS(unsafe { RtlGetVersion(version_info.as_mut_ptr() as *mut _) }) {
            String::from("")
        } else {
            let version_info = unsafe { version_info.assume_init() };
            format!(
                "{}.{}.{}",
                version_info.dwMajorVersion,
                version_info.dwMinorVersion,
                version_info.dwBuildNumber
            )
        }
    }
}

#[op]
pub fn op_platform() -> String {
    return format!("{}", env::consts::OS);
}

#[op]
pub fn op_freemem() -> String {
    return format!("{}", System::new_all().used_memory());
}

#[op]
pub fn op_totalmem() -> String {
    return format!("{}", System::new_all().total_memory());
}

#[op]
pub fn op_loadavg() -> String {
    let load_avg = System::new_all().load_average();
    return format!(
        "[{}, {}, {}]",
        load_avg.one, load_avg.five, load_avg.fifteen
    );
}

#[op]
pub fn op_dirname() -> String {
    let dir = env::current_dir().unwrap();
    return format!("{}", dir.display());
}

#[op]
pub fn op_exit(code: i32) {
    process::exit(code);
}
