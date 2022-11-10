mod os;

use colored::Colorize;
use deno_core::error::AnyError;
use deno_core::op;
use deno_core::Extension;
use duration_string::DurationString;
use std::{env, process, rc::Rc, thread, time::Instant};

#[op]
fn op_stdout(msg: String) -> Result<(), AnyError> {
    println!("{}", msg);
    Ok(())
}

#[op]
fn op_stderr(msg: String) -> Result<(), AnyError> {
    eprintln!("{}", format!("{}", msg).red());
    Ok(())
}

#[op]
fn op_sleep(ms: String) -> Result<(), AnyError> {
    thread::sleep(DurationString::from_string(ms).unwrap().into());
    Ok(())
}

#[op]
async fn op_read_file(path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op]
async fn op_write_file(path: String, contents: String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op]
fn op_remove_file(path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}

async fn exec(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path)?;
    let runjs_extension = Extension::builder()
        .ops(vec![
            op_stdout::decl(),
            op_stderr::decl(),
            op_sleep::decl(),
            op_read_file::decl(),
            op_write_file::decl(),
            op_remove_file::decl(),
            os::op_release::decl(),
            os::op_platform::decl(),
        ])
        .build();
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });
    const RUNTIME_JAVASCRIPT_CORE: &str = include_str!("./runtime/main.js");
    js_runtime
        .execute_script("[exec:runtime.js]", RUNTIME_JAVASCRIPT_CORE)
        .unwrap();

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

fn main() {
    let args: Vec<String> = env::args().collect();
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
    if let Err(error) = runtime.block_on(exec(&*format!("{}.js", filename))) {
        eprintln!("error: {}", error);
    } else {
        println!(
            "\n{} took {}",
            format!("{}.js", filename).white(),
            format!("{:.2?}", start.elapsed()).yellow()
        )
    }
}
