use colored::Colorize;
use inquire::Select;
use shell::cmd;
use std::collections::BTreeMap;

pub fn task_list(tasks: BTreeMap<String, String>) {
    let options = tasks
        .iter()
        .map(|(key, val)| format!("{}: {}", key, val))
        .collect::<Vec<_>>();

    match Select::new("Select a task to run:", options).prompt() {
        Ok(task) => {
            let key = task.split(":").collect::<Vec<_>>()[0];

            println!("{} {} `{}`", "running".green(), "task", tasks[key]);
            cmd!(&tasks[key]).run().unwrap();
        }
        Err(_) => println!("{}", "Aborting...".white()),
    }
}
