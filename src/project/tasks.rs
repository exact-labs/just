use crate::helpers;
use colored::Colorize;
use inquire::Select;
use macros::error;
use shell::cmd;
use std::collections::BTreeMap;

pub fn task_list(tasks: BTreeMap<String, String>) {
    let options = tasks.iter().map(|(key, val)| format!("({key}): '{val}'")).collect::<Vec<_>>();

    match Select::new("Select a task to run:", options).prompt() {
        Ok(task) => {
            let key = helpers::trim_start_end(task.split(":").collect::<Vec<_>>()[0]);
            println!("\n{} task {}", "running".green(), key.bold());
            println!("{} {}\n", "»".white(), tasks[key]);

            for command in &tasks[key].split("&&").collect::<Vec<&str>>() {
                if let Err(error) = cmd!(command.trim()).run() {
                    error!("{:?}", error);
                }
            }
        }
        Err(_) => println!("{}", "Aborting...".white()),
    }
}
