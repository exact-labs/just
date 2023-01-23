use colored::Colorize;
use inquire::{Confirm, Select, Text};
use std::fs::File;
use std::io::Write;

fn create_error(name: &str) {
    println!("An error happened when asking for {name}, try again later.");
    std::fs::remove_file("package.yml").unwrap();
    std::process::exit(1);
}

pub fn create_project() {
    println!("This utility will walk you through creating a package.yml file.\n");

    let mut file = File::create("package.yml").unwrap();
    let current_dir = std::env::current_dir().unwrap();
    writeln!(&mut file, "info:").unwrap();

    let name = Text::new("package name:").with_default(&current_dir.file_name().unwrap().to_str().unwrap().to_string()).prompt();
    let version = Text::new("version:").with_default("1.0.0").prompt();
    let description = Text::new("description:").prompt();
    let index = Text::new("entry point:").with_default("index.js").prompt();
    let url = Text::new("project url:").prompt();
    let repo = Text::new("repository url:").prompt();
    let author = Text::new("author:").prompt();
    let license = Text::new("license:").with_default("MIT").prompt();
    let public = Confirm::new("public:").with_default(true).prompt();
    let group = Select::new("group:", vec!["local", "net", "both"]).prompt();

    match name {
        Ok(name) => writeln!(&mut file, "  name: {name}").unwrap(),
        Err(_) => create_error("package name"),
    }
    match version {
        Ok(version) => writeln!(&mut file, "  version: {version}").unwrap(),
        Err(_) => create_error("version"),
    }
    match description {
        Ok(description) => writeln!(&mut file, "  description: {description}").unwrap(),
        Err(_) => create_error("description"),
    }
    match index {
        Ok(index) => {
            writeln!(&mut file, "  index: {index}").unwrap();
            if let Err(_) = File::create(index) {
                create_error("index");
            };
        }
        Err(_) => create_error("index"),
    }
    match url {
        Ok(url) => writeln!(&mut file, "  url: {url}").unwrap(),
        Err(_) => create_error("url"),
    }
    match repo {
        Ok(url) => writeln!(&mut file, "  repository: {url}").unwrap(),
        Err(_) => create_error("repository"),
    }
    match author {
        Ok(author) => writeln!(&mut file, "  author: {author}").unwrap(),
        Err(_) => create_error("author"),
    }
    match license {
        Ok(license) => writeln!(&mut file, "  license: {license}").unwrap(),
        Err(_) => create_error("license"),
    }
    match public {
        Ok(true) => writeln!(&mut file, "registry:\n  public: true").unwrap(),
        Ok(false) => writeln!(&mut file, "registry:\n  public: false").unwrap(),
        Err(_) => create_error("public"),
    }
    match group {
        Ok(group) => writeln!(&mut file, "  group: {group}").unwrap(),
        Err(_) => create_error("group"),
    }

    writeln!(&mut file, "tasks:\ntests:\ndependencies:").unwrap();
    println!("{}", "\n✨ success, saved package.yml".yellow())
}
