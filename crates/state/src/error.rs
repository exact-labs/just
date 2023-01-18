use colored::Colorize;
use std::process::exit;

#[macro_export]
macro_rules! error {
    ($c:expr, $v:expr) => {
        if !$c {
            $v
        }
    };
}

pub fn env(path: &str) {
    eprintln!(
        "{} {}",
        format!("[env/{path}]").bright_red(),
        "permission denied: to access system env, please run again with the --allow-env flag".red()
    );
    exit(1)
}
pub fn net(path: &str) {
    eprintln!(
        "{} {}",
        format!("[network/{path}]").bright_red(),
        "permission denied: to access the internet, please run again with the --allow-net flag".red()
    );
    exit(1)
}
pub fn cmd(path: &str) {
    eprintln!(
        "{} {}",
        format!("[spawn/{path}]").bright_red(),
        "permission denied: to run a subprocess, please run again with the --allow-cmd flag".red()
    );
    exit(1)
}
pub fn sys(path: &str) {
    eprintln!(
        "{} {}",
        format!("[system/{path}]").bright_red(),
        "permission denied: to allow system functions, please run again with the --allow-sys flag".red()
    );
    exit(1)
}
pub fn read(path: &str) {
    eprintln!(
        "{} {}",
        format!("[read/{path}]").bright_red(),
        "permission denied: to read a file, please run again with the --allow-read flag".red()
    );
    exit(1)
}
pub fn write(path: &str) {
    eprintln!(
        "{} {}",
        format!("[write/{path}]").bright_red(),
        "permission denied: to write a file, please run again with the --allow-write flag".red()
    );
    exit(1)
}
