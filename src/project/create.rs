use colored::Colorize;
use inquire::Select;
use std::io::Cursor;
use std::path::PathBuf;
use zip_extract::extract;

fn create_template(name: &str, registry: &String) {
    let target_dir = PathBuf::from(name);
    match reqwest::blocking::get(format!("{registry}/api/{}/templates/{name}.zip", env!("CARGO_PKG_VERSION").split(".").collect::<Vec<&str>>().join(""))) {
        Ok(res) => {
            if let Err(_) = extract(Cursor::new(&res.bytes().unwrap()), &target_dir, true) {
                eprintln!("{} {}", "✖".red(), "unable create template, please try again".bright_red());
            } else {
                println!("\x08{} {}", "✔".green(), format!("downloaded template {name}").green());
            }
        }
        Err(_) => {
            eprintln!("{} {}", "✖".red(), "unable create template, please try again".bright_red());
        }
    };
}

pub fn download_template(registry: &String) {
    let options = vec![
        "basic_example",
        "advanced_example",
        "benchmark",
        "sqlite",
        "webserver",
        "static_http",
        "cmd_spawn",
        "read_file",
        "utility",
        "hashing",
        "chat_server",
    ];

    match Select::new("Select a template:", options).prompt() {
        Ok(choice) => create_template(choice, registry),
        Err(_) => println!("{}", "Aborting...".white()),
    }
}
