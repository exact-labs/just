use anyhow::{Context, Error};
use macros::{crash, str};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, fs::File};

#[derive(Debug, Deserialize)]
pub struct Info {
    pub index: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub info: Info,
}

pub fn read_index(dir: std::path::Display, package: &String, version: &str) -> Project {
    let contents = match fs::read_to_string(format!("{dir}/packages/{package}/{version}/package.yml")) {
        Ok(text) => text,
        Err(_) => {
            crash!("{package}@{version} not found. Did you run 'just install'");
        }
    };

    let yaml_file: Result<Project, _> = serde_yaml::from_str(&contents);

    let parsed = match yaml_file {
        Ok(project) => project,
        Err(error) => {
            crash!("{} in package.yml", error);
        }
    };

    return parsed;
}

pub fn sha256_digest(path: &PathBuf) -> Result<String, Error> {
    let input = File::open(path)?;
    let mut reader = BufReader::new(input);

    let digest = {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };
    Ok(data_encoding::HEXLOWER.encode(digest.as_ref()))
}

pub fn get_home_dir() -> Result<PathBuf, Error> {
    let home_dir = home::home_dir().context("Unable to find home directory.")?;

    Ok(home_dir)
}

pub struct Exists;
impl Exists {
    pub fn folder(dir_name: String) -> Result<bool, Error> {
        Ok(Path::new(str!(dir_name)).is_dir())
    }
    pub fn file(file_name: String) -> Result<bool, Error> {
        Ok(Path::new(str!(file_name)).exists())
    }
}

pub fn trim_start_end(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

pub fn to_msec(maybe_time: Result<SystemTime, std::io::Error>) -> (u64, bool) {
    match maybe_time {
        Ok(time) => (
            time.duration_since(UNIX_EPOCH).map(|t| t.as_millis() as u64).unwrap_or_else(|err| err.duration().as_millis() as u64),
            true,
        ),
        Err(_) => (0, false),
    }
}

#[cfg(target_os = "linux")]
pub fn rss() -> usize {
    fn scan_int(string: &str) -> (usize, usize) {
        let mut out = 0;
        let mut idx = 0;
        let mut chars = string.chars().peekable();
        while let Some(' ') = chars.next_if_eq(&' ') {
            idx += 1;
        }
        for n in chars {
            idx += 1;
            if ('0'..='9').contains(&n) {
                out *= 10;
                out += n as usize - '0' as usize;
            } else {
                break;
            }
        }
        (out, idx)
    }

    let statm_content = if let Ok(c) = std::fs::read_to_string("/proc/self/statm") {
        c
    } else {
        return 0;
    };

    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if page_size < 0 {
        return 0;
    }

    let (_total_size_pages, idx) = scan_int(&statm_content);
    let (total_rss_pages, _) = scan_int(&statm_content[idx..]);

    total_rss_pages * page_size as usize
}

#[cfg(target_os = "macos")]
pub fn rss() -> usize {
    let mut task_info = std::mem::MaybeUninit::<libc::mach_task_basic_info_data_t>::uninit();
    let mut count = libc::MACH_TASK_BASIC_INFO_COUNT;
    let r = unsafe {
        libc::task_info(
            libc::mach_task_self(),
            libc::MACH_TASK_BASIC_INFO,
            task_info.as_mut_ptr() as libc::task_info_t,
            &mut count as *mut libc::mach_msg_type_number_t,
        )
    };
    assert_eq!(r, libc::KERN_SUCCESS);
    let task_info = unsafe { task_info.assume_init() };
    task_info.resident_size as usize
}

#[cfg(windows)]
pub fn rss() -> usize {
    use winapi::shared::minwindef::DWORD;
    use winapi::shared::minwindef::FALSE;
    use winapi::um::processthreadsapi::GetCurrentProcess;
    use winapi::um::psapi::GetProcessMemoryInfo;
    use winapi::um::psapi::PROCESS_MEMORY_COUNTERS;

    unsafe {
        let current_process = GetCurrentProcess();
        let mut pmc: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
        if GetProcessMemoryInfo(current_process, &mut pmc, std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as DWORD) != FALSE {
            pmc.WorkingSetSize
        } else {
            0
        }
    }
}
