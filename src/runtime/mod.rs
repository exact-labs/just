use crate::go;
use crate::loader;
use crate::ops;
use engine::{include_js_files, v8, Extension, JsRuntime, RuntimeOptions};
use std::rc::Rc;

fn extensions() -> Extension {
    let ops = vec![
        ops::fs::op_read_file::decl(),
        ops::fs::op_file_sha::decl(),
        ops::fs::op_read_dir::decl(),
        ops::fs::op_make_dir::decl(),
        ops::fs::op_write_file::decl(),
        ops::fs::op_remove_file::decl(),
        ops::fs::op_remove_dir::decl(),
        ops::modify::op_encode::decl(),
        ops::modify::op_encode_fast::decl(),
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
        go::external_function::decl(),
    ];

    return Extension::builder()
        .js(include_js_files!(
          prefix "[exec:runtime]",
          "main.js",
          "helpers.js",
          "core/core.js",
          "core/log.js",
          "core/go.js",
          "core/system.js",
          "core/format.js",
        ))
        .ops(ops)
        .ops(ops::core::init())
        .build();
}

pub fn import_lib(lib_name: &str) -> &str {
    return match lib_name {
        "io" => include_str!("lib/io.js"),
        "sys" => include_str!("lib/sys.js"),
        "net" => include_str!("lib/net.js"),
        _ => "",
    };
}

pub async fn repl(line: &str) -> Result<v8::Global<v8::Value>, anyhow::Error> {
    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(loader::RuntimeImport)),
        extensions: vec![extensions()],
        ..Default::default()
    });
    return js_runtime.execute_script("<repl>", line);
}

pub async fn exec(code_path: &String, code_content: String) -> Result<(), anyhow::Error> {
    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(loader::RuntimeImport)),
        extensions: vec![extensions()],
        ..Default::default()
    });

    let main_module = loader::import_prefix(code_path)?;
    let mod_id = js_runtime.load_main_module(&main_module, (!code_content.is_empty()).then(|| code_content)).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}
