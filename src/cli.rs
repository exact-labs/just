use crate::go;
use crate::helpers;
use crate::project;
use crate::runtime;
use crate::ternary;
use colored::Colorize;
use question::{Answer, Question};
use rustyline::{error::ReadlineError, Editor};
use shell::cmd;
use std::env;
use std::path::Path;
use std::time::Instant;

pub fn setup() {
    let home_dir = home::home_dir().unwrap();
    let folder_exists: bool = Path::new(helpers::string_to_static_str(format!(
        "{}/.just/packages",
        home_dir.display()
    )))
    .is_dir();

    go::init();

    if !folder_exists {
        std::fs::create_dir_all(format!("{}/.just/packages", &home_dir.display())).unwrap();
        println!("created {}/.just/packages", &home_dir.display());
    }
}

pub fn get_version(short: bool) -> String {
    return match short {
        true => format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        false => format!(
            "{} {} ({} {})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("GIT_HASH"),
            env!("BUILD_DATE")
        ),
    };
}

pub struct DependencyManager;
impl DependencyManager {
    pub fn install() {
        println!("install")
    }
    pub fn add(name: &String) {
        println!("{name}")
    }
    pub fn remove(name: &String) {
        println!("{name}")
    }
    pub fn clean() {
        println!("clean")
    }
}

pub fn project_meta() {
    let package = project::package::read();
    println!(
        "{} {} {}",
        "starting".green(),
        format!("{}", package.name),
        format!("v{}", package.version).cyan()
    );
}

pub fn run_task(task: &str) {
    let tasks = project::package::read().tasks;
    println!("{} {} `{}`", "running".green(), "task", tasks[task],);
    cmd!(&tasks[task]).run().unwrap();
}

pub fn list_tasks() {
    let tasks = project::package::read().tasks;
    project::tasks::task_list(tasks);
}

pub fn create_project_yml() {
    let exists: bool = std::path::Path::new("package.yml").exists();
    if !exists {
        project::init::create_project();
    } else {
        let answer = Question::new("overwrite project.yml?")
            .show_defaults()
            .confirm();

        ternary!(
            answer == Answer::YES,
            project::init::create_project(),
            println!("{}", "Aborting...".white())
        )
    }
}

pub fn run_exec(path: String, silent: bool) {
    let exists: bool = std::path::Path::new("package.yml").exists();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if silent {
        if let Err(error) = runtime.block_on(runtime::exec(&path)) {
            eprintln!("{}", format!("{}", error).red());
        }
    } else {
        ternary!(exists, project_meta(), {});
        let start = Instant::now();
        if let Err(error) = runtime.block_on(runtime::exec(&path)) {
            eprintln!("{}", format!("{}", error).red());
        } else {
            let path = path.split("/").collect::<Vec<_>>();

            println!(
                "\n{} took {}",
                format!("{}", path[path.len() - 1]).white(),
                format!("{:.2?}", start.elapsed()).yellow()
            )
        }
    }
}

pub fn run_repl() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut readline_editor = Editor::<()>::new();
    let mut exit_value = 0;

    println!("{}", get_version(true));
    println!("Type \".help\" for more information.");

    loop {
        let readline = readline_editor.readline("> ");
        match readline {
            Ok(line) => {
                if line == ".help" {
                    println!(
                        ".clear    Clear the screen\n.exit     Exit the REPL\n.help     Print this help message"
                    )
                } else if line == ".clear" {
                    print!("{}[2J", 27 as char);
                } else if line == ".exit" {
                    break;
                } else {
                    if let Err(error) = runtime.block_on(runtime::repl(&line)) {
                        eprintln!("{}", format!("{}", error).red());
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                exit_value += 1;
                if exit_value == 2 {
                    break;
                } else {
                    println!("(To exit, press Ctrl+C again, Ctrl+D or type .exit)");
                }
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
