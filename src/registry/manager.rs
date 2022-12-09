use crate::helpers;
use crate::ternary;
use colored::Colorize;
use flate2::read::GzDecoder;
use futures_util::StreamExt;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use tar::Archive;

#[derive(Debug, serde::Deserialize)]
struct Dist {
    version: String,
    tarball: String,
}

#[derive(Debug, serde::Deserialize)]
struct Response {
    dist: Dist,
}

fn remove_file(file: &str) {
    if let Err(_) = std::fs::remove_file(file) {
        eprintln!("{} {}", "✖".red(), "unable remove file, please try again".bright_red());
        std::process::exit(1);
    }
}
fn move_package(file: &str, name: &str, version: &str) {
    let current_dir = std::env::current_dir().expect("cannot retrive current directory");

    if !std::path::Path::new(helpers::string_to_static_str(format!("{}/packages", current_dir.display()))).is_dir() {
        std::fs::create_dir_all(format!("{}/packages", current_dir.display())).unwrap();
    }

    match File::open(file) {
        Ok(tarball) => {
            let tar = GzDecoder::new(tarball);
            let mut archive = Archive::new(tar);
            archive.unpack(format!("{}/packages/{name}/{version}", current_dir.display())).expect("failed to unpack tarball");
            remove_file(file);
        }
        Err(_) => {
            eprintln!("{} {}", "✖".red(), "unable to add package, filesystem error".bright_red());
            remove_file(file);
            std::process::exit(1);
        }
    }
}

pub async fn download(client: &reqwest::Client, url: &str, path: &str, package_info: String) -> Result<(), String> {
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("\r{} {}\n", "✖".red(), format!("failed to get from {}", &url).bright_red())))?;

    let total_size = res
        .content_length()
        .ok_or(format!("\r{} {}\n", "✖".red(), format!("failed to get content length of {}", &url).bright_red()))?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{msg}: [{bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap());
    pb.set_message(format!("{}", format!("+ {package_info}").bright_cyan()));

    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk).or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("{}", format!("+ {package_info}").bright_cyan()));
    return Ok(());
}

pub fn install() {
    println!("install")
}

pub fn add(input: &str) {
    let mut version = "".to_string();
    let name = input.split("@").collect::<Vec<&str>>()[0];
    let style = ProgressStyle::with_template("{spinner:.yellow} {msg}").unwrap().tick_strings(&[
        "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]", "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]", "",
    ]);

    let package_info = ternary!(
        input.split("@").collect::<Vec<&str>>().len() > 1,
        format!("{}@{}", input.split("@").collect::<Vec<&str>>()[0], input.split("@").collect::<Vec<&str>>()[1]),
        input.split("@").collect::<Vec<&str>>()[0].to_string()
    );

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb.set_style(style.clone());
    pb.set_message("locating...");

    match reqwest::blocking::get(format!("https://r.justjs.dev/{package_info}")) {
        Ok(res) => {
            match serde_json::from_str::<Response>(&res.text().unwrap()) {
                Ok(json) => {
                    version = json.dist.version.clone();
                    pb.finish_with_message(format!("\x08{} {}", "✔".green(), format!("located package {name}@{}", json.dist.version).green()));

                    let runtime = tokio::runtime::Runtime::new().unwrap();
                    match runtime.block_on(download(&reqwest::Client::new(), &json.dist.tarball, &format!("{name}.tgz"), format!("{name}@{}", &json.dist.version))) {
                        Ok(_) => move_package(&format!("{name}.tgz"), &name, &json.dist.version),
                        Err(err) => {
                            eprint!("\r{} {}\n", "✖".red(), format!("unable to add package {}: {}", package_info, err.to_string()).bright_red());
                            std::process::exit(1);
                        }
                    };
                }
                Err(_) => {
                    pb.finish_with_message(format!("\x08{} {}", "✖".red(), format!("unable to find {}", package_info).bright_red()));
                }
            };
        }
        Err(err) => {
            eprint!("\r{} {}\n", "✖".red(), format!("unable to add package {}: {}", package_info, err.to_string()).bright_red());
            std::process::exit(1);
        }
    };

    match reqwest::blocking::get(format!("https://r.justjs.dev/dependencies/{package_info}")) {
        Ok(res) => {
            match serde_json::from_str::<HashMap<String, Vec<String>>>(&res.text().unwrap()) {
                Ok(json) => {
                    for link in &json[&version] {
                        let pb_dep = ProgressBar::new_spinner();
                        let name = link.split("/").collect::<Vec<&str>>()[3];
                        let version = link.split("/").collect::<Vec<&str>>()[5];

                        pb_dep.enable_steady_tick(std::time::Duration::from_millis(80));
                        pb_dep.set_style(style.clone());
                        pb_dep.set_message("locating...");
                        pb_dep.finish_with_message(format!("\x08{} {}", "✔".green(), format!("located dependency {name}@{}", &version).bright_green()));
                    }

                    for link in &json[&version] {
                        let name = link.split("/").collect::<Vec<&str>>()[3];
                        let version = link.split("/").collect::<Vec<&str>>()[5];
                        let runtime = tokio::runtime::Runtime::new().unwrap();

                        match runtime.block_on(download(&reqwest::Client::new(), link, &format!("{name}.tgz"), format!("{name}@{}", version))) {
                            Ok(_) => move_package(&format!("{name}.tgz"), &name, &version),
                            Err(err) => {
                                eprint!("\r{} {}\n", "✖".red(), format!("unable to add package {}: {}", package_info, err.to_string()).bright_red());
                                std::process::exit(1);
                            }
                        };
                    }
                }
                Err(_) => {}
            };
        }
        Err(err) => {
            eprint!("\r{} {}\n", "✖".red(), format!("unable to add package {}: {}", package_info, err.to_string()).bright_red());
            std::process::exit(1);
        }
    };

    // println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
}

pub fn remove(name: &String) {
    let package_dir = ternary!(
        name.split("@").collect::<Vec<&str>>().len() > 1,
        format!("{}/{}", name.split("@").collect::<Vec<&str>>()[0], name.split("@").collect::<Vec<&str>>()[1]),
        name.split("@").collect::<Vec<&str>>()[0].to_string()
    );

    let current_dir = std::env::current_dir().expect("cannot retrive current directory");
    if let Err(_) = std::fs::remove_dir_all(format!("{}/packages/{package_dir}", current_dir.display())) {
        eprintln!("{} {}", "✖".red(), format!("unable to remove {name}, is it installed?").bright_red());
        std::process::exit(1);
    } else {
        println!("\x08{} {}", "✔".green(), format!("removed package {name}").green());
    }
}

pub fn clean() {
    println!("clean")
}
