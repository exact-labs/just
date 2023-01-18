mod helpers;

use colored::Colorize;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process;

static BINARY_EXTERNAL: &'static [u8] = include_bytes!("embed/external");

pub fn init_paths() {
    match home::home_dir() {
        Some(path) => {
            let folder_exists: bool = Path::new(helpers::string_to_static_str(format!("{}/.just/bin", path.display()))).is_dir();
            let binary_exists: bool = Path::new(helpers::string_to_static_str(format!("{}/.just/bin/lib_ext.bin", path.display()))).is_file();

            if !folder_exists {
                std::fs::create_dir_all(format!("{}/.just/bin", path.display())).unwrap();
                println!("created {}/.just/bin", path.display());
            }

            let external_runtime = format!("{}/.just/bin/lib_ext.bin", path.display());

            let write_file = || {
                let mut file = File::create(external_runtime.clone()).unwrap();
                file.write_all(BINARY_EXTERNAL).unwrap();
                println!("wrote external runtime file {}", external_runtime.clone());
                file.set_permissions(std::fs::Permissions::from_mode(0o755)).unwrap();
            };

            if binary_exists {
                let sha_sum = helpers::sha256_digest(&PathBuf::from(external_runtime.clone())).unwrap();

                if env!("FILE_SHA") != sha_sum {
                    write_file();
                } else {
                    println!("external runtime for version: {} already exists", env!("CARGO_PKG_VERSION"))
                }
            } else {
                write_file();
            }
        }
        None => {
            eprintln!("{}", "Impossible to get your home dir.".red());
            process::exit(1);
        }
    }
}