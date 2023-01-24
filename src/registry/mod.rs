use crate::helpers;

use colored::Colorize;
use std::{fs::File, io::Write};

pub fn set_default(registry_url: &String, silent: bool) {
    match home::home_dir() {
        Some(path) => {
            let mut file = File::create(format!("{}/.just/default.d", path.display())).unwrap();
            file.write_all(registry_url.as_bytes()).unwrap();
            log::info!("set default: {registry_url}");
            if !silent {
                println!("{} {} {}", "✔".green(), "changed default registry to".bright_green(), registry_url.bright_yellow())
            };
        }
        None => {
            eprintln!("{}", "Impossible to get your home dir.".red());
            std::process::exit(1);
        }
    }
}

pub fn get_default() -> String {
    match home::home_dir() {
        Some(path) => {
            if !helpers::Exists::file(format!("{}/.just/default.d", path.display())).unwrap() {
                File::create(format!("{}/.just/default.d", path.display())).unwrap();
                log::info!("created {}/.just/default.d", &path.display());
            }

            match std::fs::read_to_string(format!("{}/.just/default.d", path.display())) {
                Ok(content) => {
                    log::info!("got default: {content}");
                    return content;
                }
                Err(_) => {
                    eprintln!("{} {}", "✖".red(), "unable to find default registry, please report this error".bright_red());
                    std::process::exit(1);
                }
            };
        }
        None => {
            eprintln!("{}", "Impossible to get your home dir.".red());
            std::process::exit(1);
        }
    }
}

pub mod auth;
pub mod manager;
pub mod package;
