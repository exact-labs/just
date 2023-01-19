#[cfg(target_family = "windows")]
use std::sync::Once;

use anyhow::Error;
use dirs;
use engine::op;
use macros::function_name;
use std::thread;
use std::{env, process};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemInfo {
    pub total: u64,
    pub free: u64,
    pub available: u64,
    pub buffers: u64,
    pub cached: u64,
    pub swap_total: u64,
    pub swap_free: u64,
}

fn mem_info() -> Option<MemInfo> {
    let mut mem_info = MemInfo {
        total: 0,
        free: 0,
        available: 0,
        buffers: 0,
        cached: 0,
        swap_total: 0,
        swap_free: 0,
    };
    #[cfg(target_os = "linux")]
    {
        let mut info = std::mem::MaybeUninit::uninit();
        let res = unsafe { libc::sysinfo(info.as_mut_ptr()) };
        if res == 0 {
            let info = unsafe { info.assume_init() };
            let mem_unit = info.mem_unit as u64;
            mem_info.swap_total = info.totalswap * mem_unit;
            mem_info.swap_free = info.freeswap * mem_unit;
            mem_info.total = info.totalram * mem_unit;
            mem_info.free = info.freeram * mem_unit;
            mem_info.buffers = info.bufferram * mem_unit;
        }
    }
    #[cfg(any(target_vendor = "apple"))]
    {
        let mut mib: [i32; 2] = [0, 0];
        mib[0] = libc::CTL_HW;
        mib[1] = libc::HW_MEMSIZE;
        unsafe {
            let mut size = std::mem::size_of::<u64>();
            libc::sysctl(mib.as_mut_ptr(), mib.len() as _, &mut mem_info.total as *mut _ as *mut libc::c_void, &mut size, std::ptr::null_mut(), 0);
            mem_info.total /= 1024;

            let mut xs: libc::xsw_usage = std::mem::zeroed::<libc::xsw_usage>();
            mib[0] = libc::CTL_VM;
            mib[1] = libc::VM_SWAPUSAGE;

            let mut size = std::mem::size_of::<libc::xsw_usage>();
            libc::sysctl(mib.as_mut_ptr(), mib.len() as _, &mut xs as *mut _ as *mut libc::c_void, &mut size, std::ptr::null_mut(), 0);

            mem_info.swap_total = xs.xsu_total;
            mem_info.swap_free = xs.xsu_avail;

            let mut count: u32 = libc::HOST_VM_INFO64_COUNT as _;
            let mut stat = std::mem::zeroed::<libc::vm_statistics64>();
            if libc::host_statistics64(libc::mach_host_self(), libc::HOST_VM_INFO64, &mut stat as *mut libc::vm_statistics64 as *mut _, &mut count) == libc::KERN_SUCCESS {
                let page_size = libc::sysconf(libc::_SC_PAGESIZE) as u64;
                mem_info.available = (stat.free_count as u64 + stat.inactive_count as u64) * page_size / 1024;
                mem_info.free = (stat.free_count as u64 - stat.speculative_count as u64) * page_size / 1024;
            }
        }
    }
    #[cfg(target_family = "windows")]
    unsafe {
        use std::mem;
        use winapi::shared::minwindef;
        use winapi::um::sysinfoapi;

        let mut mem_status = mem::MaybeUninit::<sysinfoapi::MEMORYSTATUSEX>::uninit();
        let length = mem::size_of::<sysinfoapi::MEMORYSTATUSEX>() as minwindef::DWORD;
        (*mem_status.as_mut_ptr()).dwLength = length;

        let result = sysinfoapi::GlobalMemoryStatusEx(mem_status.as_mut_ptr());
        if result != 0 {
            let stat = mem_status.assume_init();
            mem_info.total = stat.ullTotalPhys / 1024;
            mem_info.available = 0;
            mem_info.free = stat.ullAvailPhys / 1024;
            mem_info.cached = 0;
            mem_info.buffers = 0;
            mem_info.swap_total = (stat.ullTotalPageFile - stat.ullTotalPhys) / 1024;
            mem_info.swap_free = (stat.ullAvailPageFile - stat.ullAvailPhys) / 1024;
            if mem_info.swap_free > mem_info.swap_total {
                mem_info.swap_free = mem_info.swap_total;
            }
        }
    }

    Some(mem_info)
}

#[op]
fn os_release() -> String {
    state::get::sys(function_name!());
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
        if unsafe { libc::sysctl(mib.as_mut_ptr(), mib.len() as _, s.as_mut_ptr() as _, &mut len, std::ptr::null_mut(), 0) } == -1 {
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
            (*version_info.as_mut_ptr()).dwOSVersionInfoSize = std::mem::size_of::<RTL_OSVERSIONINFOEXW>() as u32;
        }
        if !NT_SUCCESS(unsafe { RtlGetVersion(version_info.as_mut_ptr() as *mut _) }) {
            String::from("")
        } else {
            let version_info = unsafe { version_info.assume_init() };
            format!("{}.{}.{}", version_info.dwMajorVersion, version_info.dwMinorVersion, version_info.dwBuildNumber)
        }
    }
}

#[op]
fn os_platform() -> String {
    state::get::sys(function_name!());
    format!("{}", env::consts::OS)
}

#[op]
fn os_machine() -> String {
    state::get::sys(function_name!());
    format!("{}", env::consts::ARCH)
}

#[op]
pub fn os_hostname() -> String {
    state::get::sys(function_name!());
    #[cfg(target_family = "unix")]
    unsafe {
        let buf_size = libc::sysconf(libc::_SC_HOST_NAME_MAX) as usize;
        let mut buf = vec![0u8; buf_size + 1];
        let len = buf.len();
        if libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, len) < 0 {
            return String::from("");
        }
        buf[len - 1] = 0;
        std::ffi::CStr::from_ptr(buf.as_ptr() as *const libc::c_char).to_string_lossy().to_string()
    }
    #[cfg(target_family = "windows")]
    {
        use std::ffi::OsString;
        use std::mem;
        use std::os::windows::ffi::OsStringExt;
        use winapi::shared::minwindef::MAKEWORD;
        use winapi::um::winsock2::GetHostNameW;
        use winapi::um::winsock2::WSAStartup;

        let namelen = 256;
        let mut name: Vec<u16> = vec![0u16; namelen];

        WINSOCKET_INIT.call_once(|| unsafe {
            let mut data = mem::zeroed();
            let wsa_startup_result = WSAStartup(MAKEWORD(2, 2), &mut data);
            if wsa_startup_result != 0 {
                panic!("Failed to start winsocket");
            }
        });
        let err = unsafe { GetHostNameW(name.as_mut_ptr(), namelen as libc::c_int) };

        if err == 0 {
            let len = name.iter().take_while(|&&c| c != 0).count();
            OsString::from_wide(&name[..len]).to_string_lossy().into_owned()
        } else {
            String::from("")
        }
    }
}

#[op]
fn os_homedir() -> Result<String, Error> {
    state::get::sys(function_name!());
    let dir = dirs::home_dir().unwrap();
    Ok(String::from(dir.to_string_lossy()))
}

#[op]
pub fn os_uptime() -> u64 {
    state::get::sys(function_name!());
    let uptime: u64;
    #[cfg(target_os = "linux")]
    {
        let mut info = std::mem::MaybeUninit::uninit();
        let res = unsafe { libc::sysinfo(info.as_mut_ptr()) };
        uptime = if res == 0 {
            let info = unsafe { info.assume_init() };
            info.uptime as u64
        } else {
            0
        }
    }

    #[cfg(any(target_vendor = "apple", target_os = "freebsd", target_os = "openbsd"))]
    {
        use std::mem;
        use std::time::Duration;
        use std::time::SystemTime;
        let mut request = [libc::CTL_KERN, libc::KERN_BOOTTIME];
        let mut boottime: libc::timeval = unsafe { mem::zeroed() };
        let mut size: libc::size_t = mem::size_of_val(&boottime) as libc::size_t;
        let res = unsafe { libc::sysctl(&mut request[0], 2, &mut boottime as *mut libc::timeval as *mut libc::c_void, &mut size, std::ptr::null_mut(), 0) };
        uptime = if res == 0 {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| (d - Duration::new(boottime.tv_sec as u64, boottime.tv_usec as u32 * 1000)).as_secs())
                .unwrap_or_default()
        } else {
            0
        }
    }

    #[cfg(target_family = "windows")]
    unsafe {
        uptime = winapi::um::sysinfoapi::GetTickCount64() / 1000;
    }

    uptime
}

#[op]
fn os_cpus() -> usize {
    return thread::available_parallelism().map(|p| p.get()).unwrap_or(1);
}

#[op]
fn os_memory() -> Result<Option<MemInfo>, Error> {
    state::get::sys(function_name!());
    Ok(mem_info())
}

type LoadAvg = (f64, f64, f64);
const DEFAULT_LOADAVG: LoadAvg = (0.0, 0.0, 0.0);

#[op]
pub fn os_loadavg() -> LoadAvg {
    #[cfg(target_os = "linux")]
    {
        use libc::SI_LOAD_SHIFT;

        let mut info = std::mem::MaybeUninit::uninit();
        let res = unsafe { libc::sysinfo(info.as_mut_ptr()) };
        if res == 0 {
            let info = unsafe { info.assume_init() };
            (
                info.loads[0] as f64 / (1 << SI_LOAD_SHIFT) as f64,
                info.loads[1] as f64 / (1 << SI_LOAD_SHIFT) as f64,
                info.loads[2] as f64 / (1 << SI_LOAD_SHIFT) as f64,
            )
        } else {
            DEFAULT_LOADAVG
        }
    }
    #[cfg(any(target_vendor = "apple", target_os = "freebsd", target_os = "openbsd"))]
    {
        let mut l: [f64; 3] = [0.; 3];
        if unsafe { libc::getloadavg(&mut l as *mut f64, l.len() as _) } < 3 {
            DEFAULT_LOADAVG
        } else {
            (l[0], l[1], l[2])
        }
    }
    #[cfg(target_os = "windows")]
    {
        DEFAULT_LOADAVG
    }
}

#[op]
fn os_dirname() -> Result<String, Error> {
    let dir = env::current_dir()?;
    Ok(String::from(dir.to_string_lossy()))
}

#[op]
fn os_exit(code: i32) {
    process::exit(code);
}
