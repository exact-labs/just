use colored::Colorize;
use inquire::Select;

pub fn download_template() {
    let options = vec![
        "basic example",
        "advanced example",
        "benchmark",
        "sqlite",
        "webserver",
        "static http",
        "cmd spawn",
        "read file",
        "utility",
        "hashing",
        "chat server",
    ];

    let ans = Select::new("Select a template:", options).prompt();

    match ans {
        Ok(choice) => println!("{choice}"),
        Err(_) => println!("There was an error, please try again"),
    }

    if let Err(error) = std::fs::create_dir("dirname") {
        eprintln!("{}", format!("{}", error).red());
    }
}
