use crate::logger;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Project {
    pub index: String,
}

pub fn read_index(dir: std::path::Display, package: &String, version: &String) -> Project {
    let contents =
        match fs::read_to_string(format!("{dir}/packages/{package}/{version}/package.yml")) {
            Ok(text) => text,
            Err(_) => {
                logger::error(format!(
                    "{package}@{version} not found. Did you run 'just install'"
                ));
                std::process::exit(1);
            }
        };

    let yaml_file: Result<Project, _> = serde_yaml::from_str(&contents);

    let parsed = match yaml_file {
        Ok(project) => project,
        Err(error) => {
            logger::error(format!("{} in package.yml", error));
            std::process::exit(1);
        }
    };

    return parsed;
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn trim_start_end(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
