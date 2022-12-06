use colored::Colorize;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub url: String,
    pub repository: String,
    pub license: String,
    pub index: String,
    pub tasks: BTreeMap<String, String>,
    pub tests: BTreeMap<String, String>,
    pub dependencies: BTreeMap<String, String>,
}

pub fn read() -> Package {
    let contents = fs::read_to_string("package.yml").unwrap();
    let yaml_file: Result<Package, _> = serde_yaml::from_str(&contents);

    let parsed = match yaml_file {
        Ok(project) => project,
        Err(error) => {
            eprintln!("{}", format!("{} in package.yml", error).red());
            std::process::exit(1);
        }
    };

    return parsed;
}
