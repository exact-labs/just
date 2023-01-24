use crate::helpers;
use crate::project;

use colored::Colorize;
use flate2::write::GzEncoder;
use flate2::Compression;
use indicatif::{ProgressBar, ProgressStyle};
use macros::ternary;
use std::fs::File;

#[derive(Debug, serde::Deserialize)]
struct AuthFile {
    token: String,
    access: String,
}

#[derive(Debug, serde::Deserialize)]
struct Response {
    message: serde_json::Value,
}

fn remove_tar(file: &str) {
    if let Err(_) = std::fs::remove_file(file) {
        eprintln!(" {}", "- unable to remove temporary tarfile. does it exist?".bright_red());
        std::process::exit(1);
    }
}

fn write_tar(file_name: &String) -> Result<(), std::io::Error> {
    let current_dir = std::env::current_dir().expect("cannot retrive current directory");
    log::info!("creating file: {}", file_name);
    let tar_gz = File::create(file_name)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    tar.append_dir_all(".", format!("{}", current_dir.display()))?;
    Ok(())
}

pub fn publish(registry_link: &String) {
    match home::home_dir() {
        Some(path) => {
            if !std::path::Path::new(helpers::string_to_static_str(format!("{}/.just", path.display()))).is_dir() {
                std::fs::create_dir_all(format!("{}/.just", path.display())).unwrap();
                println!("created {}/.just", path.display());
            }

            let package = project::package::read();
            let client = reqwest::blocking::Client::new();
            let file_name = format!("{}/.just/{}.tgz", path.display(), package.info.name);

            if std::path::Path::new(&file_name).is_file() {
                remove_tar(&file_name);
            }

            let auth = match std::fs::read_to_string(format!("{}/.just/credentials/{}].json", path.display(), registry_link.replace("://", "["))) {
                Ok(content) => match serde_json::from_str::<AuthFile>(&content) {
                    Ok(json) => json,
                    Err(_) => {
                        eprintln!("{} {}", "✖".red(), "unable to publish, please login with 'just login'".bright_red());
                        std::process::exit(1);
                    }
                },
                Err(_) => {
                    eprintln!("{} {}", "✖".red(), "unable to publish, please login with 'just login'".bright_red());
                    std::process::exit(1);
                }
            };

            println!(
                "{} {}@{}",
                "publishing".bright_yellow(),
                format!("{}", package.info.name).bold(),
                format!("{}", package.info.version).bold()
            );

            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(std::time::Duration::from_millis(80));
            pb.set_style(ProgressStyle::with_template("{spinner:.yellow} {msg}").unwrap().tick_strings(&[
                "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]", "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]", "",
            ]));
            pb.set_message("publishing...");

            if let Err(err) = write_tar(&file_name) {
                eprintln!("{} {}", "✖".red(), "unable to publish, please try again".bright_red());
                eprintln!(" {} {}", "-".bright_red(), err.to_string().bright_red());
                remove_tar(&file_name);
                std::process::exit(1);
            }

            let form = reqwest::blocking::multipart::Form::new()
                .text("access", auth.access)
                .text("url", package.info.url)
                .text("name", package.info.name)
                .text("index", package.info.index)
                .text("author", package.info.author)
                .text("version", package.info.version)
                .text("license", package.info.license)
                .text("group", package.registry.group)
                .text("repository", package.info.repository)
                .text("description", package.info.description)
                .text("dependencies", format!("{:?}", package.dependencies))
                .text("visibility", ternary!(package.registry.public, "public", "private"))
                .file("tarball", &file_name)
                .unwrap();

            let response = client
                .post(format!("{registry_link}/create"))
                .multipart(form)
                .header(
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_static(helpers::string_to_static_str(auth.token.clone())),
                )
                .send();

            match response {
                Ok(response) => {
                    match serde_json::from_str::<Response>(&response.text().unwrap()) {
                        Ok(json) => {
                            if &json.message["created"].to_string() == "null" {
                                let error = json.message["error"].to_string().clone();
                                pb.finish_with_message(format!(
                                    "\x08{} {}",
                                    "✖".red(),
                                    format!(
                                        "unable to publish package\n - {}",
                                        ternary!(
                                            helpers::trim_start_end(&error) == "ul",
                                            "your token might be expired, please login again with 'just login'",
                                            helpers::trim_start_end(&error)
                                        )
                                    )
                                    .bright_red()
                                ));
                                remove_tar(&file_name);
                                std::process::exit(1);
                            } else {
                                pb.finish_with_message(format!("\x08{} {}", "✔".green(), format!("created package {}", &json.message["created"]).bright_green()));
                                remove_tar(&file_name);
                            }
                        }
                        Err(_) => {
                            eprint!("\r{} {}\n", "✖".red(), "unable to publish package, please try again".bright_red());
                            remove_tar(&file_name);
                            std::process::exit(1);
                        }
                    };
                }
                Err(err) => {
                    eprint!("\r{} {}\n", "✖".red(), format!("unable to publish package: {}", err.to_string()).bright_red());
                    remove_tar(&file_name);
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
