use crate::helpers;
use colored::Colorize;
use inquire::Select;
use macros::error;
use shell::cmd;
use std::collections::BTreeMap;

pub fn test_list(tests: BTreeMap<String, String>) {
    let options = tests.iter().map(|(key, val)| format!("({key}): '{val}'")).collect::<Vec<_>>();

    match Select::new("Select a test to run:", options).prompt() {
        Ok(test) => {
            let key = helpers::trim_start_end(test.split(":").collect::<Vec<_>>()[0]);
            println!("\n{} test {}", "running".green(), key.bold());
            println!("{} {}\n", "»".white(), tests[key]);

            for command in &tests[key].split("&&").collect::<Vec<&str>>() {
                if let Err(error) = cmd!(command.trim()).run() {
                    error!("{:?}", error);
                }
            }
        }
        Err(_) => println!("{}", "Aborting...".white()),
    }
}
