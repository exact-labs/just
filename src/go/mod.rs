mod helpers;

use colored::Colorize;
use deno_core::op;
use shell::cmd;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process;

static BINARY_EXTERNAL: &'static [u8] = include_bytes!("embed/external");

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn init() {
    match home::home_dir() {
        Some(path) => {
            let exists: bool = Path::new(helpers::string_to_static_str(format!(
                "{}/.core_js",
                path.display()
            )))
            .is_dir();

            if !exists {
                std::fs::create_dir_all(format!("{}/.core_js", path.display())).unwrap();
                println!("created {}/.core_js", path.display());
            }

            let external_runtime = format!("{}/.core_js/external", path.display());
            let sha_sum = helpers::sha256_digest(&PathBuf::from(external_runtime.clone())).unwrap();

            if env!("FILE_SHA") != sha_sum {
                let mut file = File::create(external_runtime.clone()).unwrap();
                file.write_all(BINARY_EXTERNAL).unwrap();
                println!("wrote external runtime file {}", external_runtime.clone());
                file.set_permissions(std::fs::Permissions::from_mode(0o755))
                    .unwrap();
            } else {
                println!(
                    "external runtime for version: {} already exists\nhash: {}",
                    env!("CARGO_PKG_VERSION"),
                    env!("FILE_SHA")
                )
            }
        }
        None => {
            eprintln!("{}", "Impossible to get your home dir.".red());
            process::exit(0x0100);
        }
    }
}

#[op]
pub fn run_ext_func(cmd: String) -> String {
    return cmd!(string_to_static_str(format!(
        "{}/.core_js/external -run=\"{cmd}\"",
        home::home_dir().unwrap().display()
    )))
    .stdout_utf8()
    .unwrap();
}
