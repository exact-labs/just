mod cmd;
mod core;
mod db;
mod fs;
mod go;
mod http;
mod modify;
mod os;
mod project;
mod serve;

use clap::{Parser, Subcommand};
use colored::Colorize;
use deno_core::error::AnyError;
use deno_core::include_js_files;
use deno_core::serde_v8;
use deno_core::Extension;
use question;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use shell::cmd;
use std::{env, rc::Rc, time::Instant};

macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {
            $v
        } else {
            $v1
        }
    };
}

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
            core::op_version::decl(),
            fs::op_read_file::decl(),
            fs::op_read_dir::decl(),
            fs::op_write_file::decl(),
            fs::op_remove_file::decl(),
            modify::op_encode::decl(),
            modify::op_encode_fast::decl(),
            core::op_id::decl(),
            core::op_escape::decl(),
            core::op_packages_dir::decl(),
            core::op_stdout::decl(),
            core::op_stderr::decl(),
            core::op_info::decl(),
            core::op_sleep::decl(),
            cmd::op_exec::decl(),
            cmd::op_spawn::decl(),
            os::op_env_get::decl(),
            os::op_env_set::decl(),
            os::op_machine::decl(),
            os::op_hostname::decl(),
            os::op_homedir::decl(),
            os::op_release::decl(),
            os::op_platform::decl(),
            os::op_cpus::decl(),
            os::op_uptime::decl(),
            os::op_freemem::decl(),
            os::op_totalmem::decl(),
            os::op_loadavg::decl(),
            os::op_dirname::decl(),
            os::op_exit::decl(),
            http::op_get::decl(),
            http::op_post::decl(),
            serve::op_static::decl(),
            serve::op_static_test::decl(),
            db::op_db_init::decl(),
            db::op_db_create::decl(),
            db::op_db_exec::decl(),
            db::op_db_insert::decl(),
            db::op_db_query::decl(),
            db::op_db_delete::decl(),
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
        let answer = question::Question::new("overwrite project.yml?")
            .show_defaults()
            .confirm();

        if answer == question::Answer::YES {
            project::init::create_project();
        } else {
            println!("Aborting...");
        }
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
        println!("{}", get_version(false))
    } else {
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
}
