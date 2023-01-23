use crate::helpers;

use anyhow::Error as AnyError;
use engine::{op, OpDecl};
use macros::{function_name, scaffold};
use serde::{Deserialize, Serialize};
use std::io::Error;
use std::os::unix::fs::PermissionsExt;
use std::{fs, path::Path, path::PathBuf};

pub fn init() -> Vec<OpDecl> {
    vec![
        chmod::decl(),
        read_file::decl(),
        write_file::decl(),
        remove_file::decl(),
        dir_list::decl(),
        make_dir::decl(),
        remove_dir::decl(),
        file_sha::decl(),
        file_stat::decl(),
        file_exists::decl(),
        dir_exists::decl(),
    ]
}

#[op]
fn file_exists(file_name: String) -> Result<bool, AnyError> {
    Ok(Path::new(helpers::string_to_static_str(file_name)).exists())
}

#[op]
fn dir_exists(dir_name: String) -> Result<bool, AnyError> {
    Ok(Path::new(helpers::string_to_static_str(dir_name)).is_dir())
}

#[op]
fn chmod(path: String, mode: u32) -> Result<(), AnyError> {
    state::get::write(function_name!());
    Ok(fs::set_permissions(path, fs::Permissions::from_mode(mode))?)
}

#[op]
fn file_sha(path: String) -> Result<String, AnyError> {
    state::get::read(function_name!());
    Ok(helpers::sha256_digest(&PathBuf::from(path.clone()))?)
}

#[op]
async fn read_file(path: String) -> Result<Vec<u8>, AnyError> {
    state::get::read(function_name!());
    Ok(tokio::fs::read(path).await?)
}

#[op]
async fn write_file(path: String, bytes: Vec<u8>) -> Result<(), AnyError> {
    state::get::write(function_name!());
    tokio::fs::write(path, bytes).await?;
    Ok(())
}

scaffold! {
  pub struct FsStat {
    is_file: bool,
    is_directory: bool,
    is_symlink: bool,
    size: u64,
    mtime_set: bool,
    mtime: u64,
    atime_set: bool,
    atime: u64,
    birthtime_set: bool,
    birthtime: u64,
    dev: u64,
    ino: u64,
    mode: u32,
    nlink: u64,
    uid: u32,
    gid: u32,
    rdev: u64,
    blksize: u64,
    blocks: u64,
  }
}

#[inline(always)]
fn get_stat(metadata: std::fs::Metadata) -> FsStat {
    state::get::read(function_name!());
    macro_rules! usm {
        ($member:ident) => {{
            #[cfg(unix)]
            {
                metadata.$member()
            }
            #[cfg(not(unix))]
            {
                0
            }
        }};
    }

    #[cfg(unix)]
    use std::os::unix::fs::MetadataExt;
    let (mtime, mtime_set) = helpers::to_msec(metadata.modified());
    let (atime, atime_set) = helpers::to_msec(metadata.accessed());
    let (birthtime, birthtime_set) = helpers::to_msec(metadata.created());

    FsStat {
        is_file: metadata.is_file(),
        is_directory: metadata.is_dir(),
        is_symlink: metadata.file_type().is_symlink(),
        size: metadata.len(),
        mtime_set,
        mtime,
        atime_set,
        atime,
        birthtime_set,
        birthtime,
        dev: usm!(dev),
        ino: usm!(ino),
        mode: usm!(mode),
        nlink: usm!(nlink),
        uid: usm!(uid),
        gid: usm!(gid),
        rdev: usm!(rdev),
        blksize: usm!(blksize),
        blocks: usm!(blocks),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatArgs {
    path: String,
    lstat: bool,
}

#[op]
async fn file_stat(args: StatArgs) -> Result<FsStat, AnyError> {
    state::get::read(function_name!());
    let path = PathBuf::from(&args.path);
    let lstat = args.lstat;

    tokio::task::spawn_blocking(move || {
        log::debug!("op_stat_async {} {}", path.display(), lstat);
        let err_mapper = |err: Error| Error::new(err.kind(), format!("{}, stat '{}'", err, path.display()));
        let metadata = if lstat {
            std::fs::symlink_metadata(&path).map_err(err_mapper)?
        } else {
            std::fs::metadata(&path).map_err(err_mapper)?
        };
        Ok(get_stat(metadata))
    })
    .await
    .unwrap()
}

#[op]
async fn remove_file(path: String) -> Result<(), AnyError> {
    state::get::write(function_name!());
    tokio::fs::remove_file(path).await?;
    Ok(())
}

#[op]
fn dir_list(path: String) -> Vec<String> {
    state::get::read(function_name!());
    let mut vec = Vec::new();
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        vec.push(format!("{}", path.unwrap().path().display()));
    }

    return vec;
}

#[op]
async fn make_dir(path: String) -> Result<(), AnyError> {
    state::get::write(function_name!());
    tokio::fs::create_dir_all(path).await?;
    Ok(())
}

#[op]
async fn remove_dir(path: String) -> Result<(), AnyError> {
    state::get::write(function_name!());
    tokio::fs::remove_dir_all(path).await?;
    Ok(())
}
