use crate::helpers;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::{min_length, Password, PasswordDisplayMode, Text};
use std::io::Write;

#[derive(Debug, serde::Deserialize)]
struct Record {
    id: String,
}

#[derive(Debug, serde::Deserialize)]
struct Response {
    token: String,
    record: Record,
}

pub fn login(registry_link: &String) {
    match home::home_dir() {
        Some(path) => {
            if !helpers::Exists::folder(format!("{}/.just", path.display())).unwrap() {
                std::fs::create_dir_all(format!("{}/.just", path.display())).unwrap();
                println!("created {}/.just", &path.display());
            }

            if !helpers::Exists::folder(format!("{}/.just/credentials", path.display())).unwrap() {
                std::fs::create_dir_all(format!("{}/.just/credentials", path.display())).unwrap();
                println!("created {}/.just/credentials", &path.display());
            }

            println!("logging into {registry_link}");

            let identity_string: String;
            let password_string: String;
            let client = reqwest::blocking::Client::new();
            let identity = Text::new("identity:").prompt();
            let password = Password::new("password:")
                .with_display_toggle_enabled()
                .with_display_mode(PasswordDisplayMode::Masked)
                .with_validator(min_length!(8))
                .without_confirmation()
                .prompt();

            match identity {
                Ok(value) => identity_string = value.clone(),
                Err(_) => std::process::exit(1),
            };

            match password {
                Ok(value) => password_string = value.clone(),
                Err(_) => std::process::exit(1),
            };

            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(std::time::Duration::from_millis(80));
            pb.set_style(ProgressStyle::with_template("{spinner:.yellow} {msg}").unwrap().tick_strings(&[
                "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]", "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]", "",
            ]));
            pb.set_message("logging in...");

            let response = client
                .post(format!("{registry_link}/api/collections/just_auth_system/auth-with-password"))
                .body(format!("{{\"identity\":\"{identity_string}\",\"password\":\"{password_string}\"}}"))
                .header(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"))
                .send();

            match response {
                Ok(response) => {
                    match serde_json::from_str::<Response>(&response.text().unwrap()) {
                        Ok(json) => {
                            let mut file = std::fs::File::create(format!("{}/.just/credentials/{}].json", path.display(), registry_link.replace("://", "["))).unwrap();
                            file.write_all(format!("{{\"token\":\"{}\",\"access\":\"{}\"}}", json.token, json.record.id).as_bytes()).unwrap();
                            pb.finish_with_message(format!("\x08{} {} {}", "✔".green(), "logged in".bright_green(), format!("({})", json.record.id).white()));
                        }
                        Err(_) => {
                            eprint!("\r{} {}\n", "✖".red(), "unable to login, invalid username or password".bright_red());
                            std::process::exit(1);
                        }
                    };
                }
                Err(err) => eprint!("\r{} {}\n", "✖".red(), format!("unable to login: {}", err.to_string()).bright_red()),
            };
        }
        None => {
            eprintln!("{}", "Impossible to get your home dir.".red());
            std::process::exit(1);
        }
    }
}

pub fn verify(registry_link: &String) {
    println!("{registry_link}");
}

pub fn logout() {
    match home::home_dir() {
        Some(path) => {
            if let Err(_) = std::fs::remove_file(format!("{}/.just/credentials.json", path.display())) {
                eprintln!("{} {}", "unable to logout, no token file".red(), "(are you logged in?)".bright_red());
            } else {
                println!("{}", "logged out".green())
            }
        }
        None => {
            eprintln!("{}", "Impossible to get your home dir.".red());
            std::process::exit(1);
        }
    }
}
