use crate::go;
use crate::loader;
use crate::ops;
use deno_core::{error::AnyError, include_js_files, Extension};
use std::rc::Rc;

const RUNTIME_JAVASCRIPT_CORE: &str = include_str!("./main.js");

fn extensions() -> deno_core::Extension {
    return Extension::builder()
        .js(include_js_files!(
          prefix "[exec:runtime]",
          "util/core.js",
          "util/cli.js",
          "util/ext.js",
          "util/cmd.js",
          "util/db.js",
          "util/native.js",
          "util/string.js",
          "util/http.js",
          "util/extra.js",
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
            ops::core::op_get_package::decl(),
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

pub async fn repl(line: &str) -> Result<deno_core::v8::Global<deno_core::v8::Value>, AnyError> {
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(loader::RuntimeImport)),
        extensions: vec![extensions()],
        ..Default::default()
    });
    js_runtime
        .execute_script("[exec:runtime]", RUNTIME_JAVASCRIPT_CORE)
        .unwrap();
    return js_runtime.execute_script("<repl>", line);
}

pub async fn exec(code_path: &String) -> Result<(), AnyError> {
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(loader::RuntimeImport)),
        extensions: vec![extensions()],
        ..Default::default()
    });
    js_runtime
        .execute_script("[exec:runtime]", RUNTIME_JAVASCRIPT_CORE)
        .unwrap();

    let main_module = loader::import_prefix(code_path)?;
    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}
