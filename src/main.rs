mod go;
mod macros;
mod ops;
mod project;

use clap::{Parser, Subcommand};
use colored::Colorize;
use deno_core::{error::AnyError, include_js_files, serde_v8, Extension};
use question::{Answer, Question};
use rustyline::{error::ReadlineError, Editor};
use shell::cmd;
use std::{env, process, rc::Rc, time::Instant};

const RUNTIME_JAVASCRIPT_CORE: &str = include_str!("./runtime/main.js");

fn get_version(short: bool) -> String {
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

fn project_meta() {
    let package = project::package::read();
    println!(
        "{} {} {}",
        "Starting".green().bold(),
        format!("{}", package.name).white(),
        format!("v{}", package.version).cyan()
    );
}

fn extensions() -> deno_core::Extension {
    return Extension::builder()
        .js(include_js_files!(
          prefix "runtime/util",
          "runtime/util/core.js",
          "runtime/util/cli.js",
          "runtime/util/ext.js",
          "runtime/util/cmd.js",
          "runtime/util/db.js",
          "runtime/util/native.js",
          "runtime/util/string.js",
          "runtime/util/http.js",
          "runtime/util/extra.js",
        ))
        .ops(vec![
            ops::core::op_version::decl(),
            ops::fs::op_read_file::decl(),
            ops::fs::op_read_dir::decl(),
            ops::fs::op_write_file::decl(),
            ops::fs::op_remove_file::decl(),
            ops::modify::op_encode::decl(),
            ops::modify::op_encode_fast::decl(),
            ops::core::op_id::decl(),
            ops::core::op_escape::decl(),
            ops::core::op_packages_dir::decl(),
            ops::core::op_stdout::decl(),
            ops::core::op_stderr::decl(),
            ops::core::op_info::decl(),
            ops::core::op_sleep::decl(),
            ops::cmd::op_exec::decl(),
            ops::cmd::op_spawn::decl(),
            ops::os::op_env_get::decl(),
            ops::os::op_env_set::decl(),
            ops::os::op_machine::decl(),
            ops::os::op_hostname::decl(),
            ops::os::op_homedir::decl(),
            ops::os::op_release::decl(),
            ops::os::op_platform::decl(),
            ops::os::op_cpus::decl(),
            ops::os::op_uptime::decl(),
            ops::os::op_freemem::decl(),
            ops::os::op_totalmem::decl(),
            ops::os::op_loadavg::decl(),
            ops::os::op_dirname::decl(),
            ops::os::op_exit::decl(),
            ops::http::op_get::decl(),
            ops::http::op_post::decl(),
            ops::serve::op_static::decl(),
            ops::serve::op_static_test::decl(),
            ops::db::op_db_init::decl(),
            ops::db::op_db_create::decl(),
            ops::db::op_db_exec::decl(),
            ops::db::op_db_insert::decl(),
            ops::db::op_db_query::decl(),
            ops::db::op_db_delete::decl(),
            go::run_ext_func::decl(),
        ])
        .build();
}

async fn exec(file_name: &str) -> Result<(), AnyError> {
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![extensions()],
        ..Default::default()
    });
    js_runtime
        .execute_script("[exec:runtime]", RUNTIME_JAVASCRIPT_CORE)
        .unwrap();

    let main_module = deno_core::resolve_path(file_name)?;
    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

async fn repl(line: &str) -> Result<deno_core::v8::Global<deno_core::v8::Value>, AnyError> {
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![extensions()],
        ..Default::default()
    });
    js_runtime
        .execute_script("[exec:runtime]", RUNTIME_JAVASCRIPT_CORE)
        .unwrap();
    return js_runtime.execute_script("<repl>", line);
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Print version information
    #[arg(short, long)]
    version: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup for executing external modules
    Setup,
    /// Bundle module and dependencies into single file
    Bundle,
    /// Compile the script into a self contained executable
    Compile,
    /// Format source files
    Fmt,
    /// Initialize a new package.yml
    Init,
    /// Initialize a new project
    Create,
    /// Run a task defined in project.yml
    Task {
        #[command()]
        task: String,
    },
    /// List all tasks in project.yml
    Tasks,
    /// Start the index script
    Start {
        #[arg(short, long)]
        silent: bool,
    },
    /// Run a javascript program
    Run {
        #[arg(short, long)]
        silent: bool,

        #[command()]
        filename: String,
    },
}

fn start_repl() {
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
                    if let Err(error) = runtime.block_on(repl(&*line)) {
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

fn run_task(task: &str) {
    let tasks = project::package::read().tasks;
    println!(
        "{} {} `{}`",
        "Running".green().bold(),
        "task".white(),
        tasks[task],
    );
    cmd!(&tasks[task]).run().unwrap();
}

fn list_tasks() {
    let tasks = project::package::read().tasks;
    project::tasks::task_list(tasks);
}

fn create_project_yml() {
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

fn start_exec(filename: String, silent: bool) {
    let exists: bool = std::path::Path::new("package.yml").exists();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if silent {
        if let Err(error) = runtime.block_on(exec(&*filename)) {
            eprintln!("{}", format!("{}", error).red());
        }
    } else {
        ternary!(exists, project_meta(), {});
        let start = Instant::now();
        if let Err(error) = runtime.block_on(exec(&*filename)) {
            eprintln!("{}", format!("{}", error).red());
        } else {
            println!(
                "\n{} took {}",
                format!("{filename}").white(),
                format!("{:.2?}", start.elapsed()).yellow()
            )
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.version {
        println!("{}", get_version(false));
        process::exit(0);
    }

    match &cli.command {
        Some(Commands::Setup) => go::init(),
        Some(Commands::Init) => create_project_yml(),
        Some(Commands::Tasks) => list_tasks(),
        Some(Commands::Task { task }) => run_task(task),
        Some(Commands::Create) => project::create::download_template(),
        Some(Commands::Fmt) => println!("fmt (wip)"),
        Some(Commands::Compile) => println!("compile (wip)"),
        Some(Commands::Bundle) => println!("bundle (wip)"),
        Some(Commands::Run { silent, filename }) => start_exec(filename.to_string(), *silent),
        Some(Commands::Start { silent }) => start_exec(project::package::read().index, *silent),
        None => start_repl(),
    }
}
