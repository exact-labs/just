use colored::Colorize;

pub fn error(content: String) {
    eprintln!("{}", format!("{}", content).red());
}
