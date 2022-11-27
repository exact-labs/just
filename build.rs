use chrono::Datelike;
use std::process::Command;

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

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=BUILD_DATE={}-{}-{}", year, month, day);
}
