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
        format!("({path})").bright_red(),
        "[ENV] Uncaught Permission Denied: to access system env, please run again with the --allow-env flag".red()
    );
    exit(1)
}
pub fn net(path: &str) {
    eprintln!(
        "{} {}",
        format!("({path})").bright_red(),
        "[NETWORK] Permission Denied: to access the internet, please run again with the --allow-net flag".red()
    );
    exit(1)
}
pub fn cmd(path: &str) {
    eprintln!(
        "{} {}",
        format!("({path})").bright_red(),
        "[SPAWN] Permission Denied: to run a subprocess, please run again with the --allow-cmd flag".red()
    );
    exit(1)
}
pub fn sys(path: &str) {
    eprintln!(
        "{} {}",
        format!("({path})").bright_red(),
        "[SYSTEM] Permission Denied: to allow system functions, please run again with the --allow-sys flag".red()
    );
    exit(1)
}
pub fn read(path: &str) {
    eprintln!(
        "{} {}",
        format!("({path})").bright_red(),
        "[READ] Permission Denied: to read a file, please run again with the --allow-read flag".red()
    );
    exit(1)
}
pub fn write(path: &str) {
    eprintln!(
        "{} {}",
        format!("({path})").bright_red(),
        "[WRITE] Permission Denied: to write a file, please run again with the --allow-write flag".red()
    );
    exit(1)
}
