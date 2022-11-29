use colored::Colorize;
use inquire::Select;
use shell::cmd;
use std::collections::BTreeMap;

pub fn task_list(tasks: BTreeMap<String, String>) {
    let options = tasks
        .iter()
        .map(|(key, val)| format!("{}: {}", key, val))
        .collect::<Vec<_>>();

    let ans = Select::new("Select a task to run:", options).prompt();

    match ans {
        Ok(task) => {
            let key = task.split(":").collect::<Vec<_>>()[0];

            println!(
                "{} {} `{}`",
                "Running".green().bold(),
                "task".white(),
                tasks[key]
            );
            cmd!(&tasks[key]).run().unwrap();
        }
        Err(_) => println!("There was an error, please try again"),
    }
}
