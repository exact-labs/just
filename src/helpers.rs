use colored::Colorize;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Project {
    pub index: String,
}

pub fn read_index(dir: String) -> Project {
    let contents = fs::read_to_string(format!("{dir}/package.yml")).unwrap();
    let yaml_file: Result<Project, _> = serde_yaml::from_str(&contents);

    let parsed = match yaml_file {
        Ok(project) => project,
        Err(error) => {
            eprintln!("{}", format!("{} in package.yml", error).red());
            std::process::exit(1);
        }
    };

    return parsed;
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
