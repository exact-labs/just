mod cmd;
mod core;
mod db;
mod fs;
mod go;
mod http;
mod modify;
mod os;
mod serve;

use clap::{Parser, Subcommand};
use colored::Colorize;
use deno_core::error::AnyError;
use deno_core::include_js_files;
use deno_core::serde_v8;
use deno_core::Extension;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::{env, rc::Rc, time::Instant};

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
    Build,
    Run {
        #[arg(short, long)]
        silent: bool,

        #[command()]
        filename: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    go::init();

    if cli.version {
        println!("{}", get_version(false))
    } else {
        match &cli.command {
            Some(Commands::Build) => {
                println!("build");
            }
            Some(Commands::Run { silent, filename }) => {
                if *silent {
                    if let Err(error) = runtime.block_on(exec(&*filename)) {
                        eprintln!("{}", format!("{}", error).red());
                    }
                } else {
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
            None => {
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
        }
    }
}
