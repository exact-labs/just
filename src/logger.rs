use colored::Colorize;

pub fn error(content: String) {
    eprintln!("{}", content.to_string().red());
}
