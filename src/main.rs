mod http;
mod os;

use crate::serde_v8::from_v8;
use colored::Colorize;
use deno_core::error::AnyError;
use deno_core::op;
use deno_core::serde_v8;
use deno_core::v8;
use deno_core::Extension;
use duration_string::DurationString;
use std::{env, process, rc::Rc, thread, time::Instant};
use v_htmlescape::escape;

#[op(v8)]
fn op_encode<'a>(scope: &mut v8::HandleScope, text: serde_v8::Value) -> serde_v8::Value<'a> {
    let text = v8::Local::<v8::String>::try_from(text.v8_value).unwrap();
    let text_str = serde_v8::to_utf8(text, scope);
    let bytes = text_str.into_bytes();
    let len = bytes.len();
    let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(bytes).make_shared();
    let buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
    let u8array = v8::Uint8Array::new(scope, buffer, 0, len).unwrap();
    from_v8(scope, u8array.into()).unwrap()
}

#[op(v8)]
fn op_encode_fast<'a>(
    scope: &mut v8::HandleScope,
    text: serde_v8::Value<'a>,
) -> serde_v8::Value<'a> {
    let s = v8::Local::<v8::String>::try_from(text.v8_value).unwrap();
    let len = s.length();
    let capacity = (len as f64 * 1.2) as usize;
    let mut buf = Vec::with_capacity(capacity);
    let mut nchars = 0;
    let data = buf.as_mut_ptr();
    let length = s.write_utf8(
        scope,
        unsafe { std::slice::from_raw_parts_mut(data, len) },
        Some(&mut nchars),
        v8::WriteOptions::NO_NULL_TERMINATION | v8::WriteOptions::REPLACE_INVALID_UTF8,
    );
    unsafe { buf.set_len(length) };
    let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(buf).make_shared();
    let buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
    from_v8(scope, buffer.into()).unwrap()
}

#[op]
fn op_escape(text: String) -> Result<String, AnyError> {
    Ok(escape(&text).to_string())
}

#[op]
fn op_packages_dir() -> String {
    let dir = env::current_dir().unwrap();
    return format!("{}/packages", dir.display());
}

#[op]
fn op_stdout(msg: String) -> Result<(), AnyError> {
    print!("{}\n", msg);
    Ok(())
}

#[op]
fn op_stderr(msg: String) -> Result<(), AnyError> {
    eprint!("{}\n", format!("{}", msg).red());
    Ok(())
}

#[op]
fn op_info(msg: String) -> Result<(), AnyError> {
    print!("{}\n", format!("{}", msg).cyan());
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
            op_packages_dir::decl(),
            op_stdout::decl(),
            op_stderr::decl(),
            op_info::decl(),
            op_sleep::decl(),
            op_read_file::decl(),
            op_write_file::decl(),
            op_remove_file::decl(),
            op_encode::decl(),
            op_encode_fast::decl(),
            op_escape::decl(),
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
            http::op_fetch::decl(),
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
