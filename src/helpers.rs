use crate::logger;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::{fs, fs::File};

#[derive(Debug, Deserialize)]
pub struct Info {
    pub index: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub info: Info,
}

pub fn read_index(dir: std::path::Display, package: &String, version: &str) -> Project {
    let contents = match fs::read_to_string(format!("{dir}/packages/{package}/{version}/package.yml")) {
        Ok(text) => text,
        Err(_) => {
            logger::error(format!("{package}@{version} not found. Did you run 'just install'"));
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

pub fn sha256_digest(path: &PathBuf) -> Result<String, anyhow::Error> {
    let input = File::open(path)?;
    let mut reader = BufReader::new(input);

    let digest = {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };
    Ok(data_encoding::HEXLOWER.encode(digest.as_ref()))
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
