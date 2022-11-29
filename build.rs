use chrono::Datelike;
use deno_core::error::AnyError;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::process::Command;

fn sha256_digest(path: &PathBuf) -> Result<String, AnyError> {
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

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "--short=10", "HEAD"])
        .output()
        .unwrap();

    let git_hash = String::from_utf8(output.stdout).unwrap();
    let current_date = chrono::Utc::now();
    let year = current_date.year();
    let month = current_date.month();
    let day = current_date.day();

    println!(
        "cargo:rustc-env=FILE_SHA={}",
        sha256_digest(&PathBuf::from("src/go/embed/external")).unwrap()
    );

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=BUILD_DATE={}-{}-{}", year, month, day);
}
