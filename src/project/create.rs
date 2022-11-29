use colored::Colorize;
use inquire::Select;

fn create_template(name: &str) {
    if let Err(error) = std::fs::create_dir(name) {
        eprintln!("{}", format!("{}", error).red());
    }
}

pub fn download_template() {
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
        Ok(choice) => create_template(choice),
        Err(_) => println!("{}", "Aborting...".white()),
    }
}
