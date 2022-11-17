mod cmd;
mod core;
mod db;
mod fs;
mod http;
mod modify;
mod os;
mod serve;

use colored::Colorize;
use deno_core::error::AnyError;
use deno_core::include_js_files;
use deno_core::op;
use deno_core::serde_v8;
use deno_core::Extension;
use std::{env, process, rc::Rc, time::Instant};

#[op]
fn op_version() -> String {
    return format!("{}", env!("CARGO_PKG_VERSION"));
}

async fn exec(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path)?;
    let runjs_extension = Extension::builder()
        .js(include_js_files!(
          prefix "runtime/util",
          "runtime/util/core.js",
          "runtime/util/cli.js",
          "runtime/util/cmd.js",
          "runtime/util/db.js",
          "runtime/util/native.js",
          "runtime/util/string.js",
          "runtime/util/http.js",
          "runtime/util/extra.js",
        ))
        .ops(vec![
            op_version::decl(),
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
        ])
        .build();
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });
    const RUNTIME_JAVASCRIPT_CORE: &str = include_str!("./runtime/main.js");
    js_runtime
        .execute_script("[exec:runtime]", RUNTIME_JAVASCRIPT_CORE)
        .unwrap();

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "--version" || args[1] == "-v") {
        println!(
            "{} {} ({} {})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("GIT_HASH"),
            env!("BUILD_DATE")
        );
        process::exit(1);
    }

    let filename = match args.len() {
        1 => {
            eprintln!("{}", "Please specify a script to run.".yellow());
            process::exit(0x0100);
        }
        2 => args[1].split(".").collect::<Vec<_>>().join("."),
        _ => {
            eprintln!("{}", "Too many arguments.".red());
            process::exit(0x0100);
        }
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let start = Instant::now();
    if let Err(error) = runtime.block_on(exec(&*format!("{}.{}", filename, "js"))) {
        eprintln!("error: {}", error);
    } else {
        println!(
            "\n{} took {}",
            format!("{}.{}", filename, "js").white(),
            format!("{:.2?}", start.elapsed()).yellow()
        )
    }
}
