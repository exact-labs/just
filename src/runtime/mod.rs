use crate::loader;
use crate::ops;

use engine::{include_js_files, serde_json, serde_json::json, v8, Extension, JsRuntime, RuntimeOptions};
use state::{permissions, permissions::Permissions};
use std::rc::Rc;
use std::thread;

#[derive(Clone)]
pub struct BootstrapOptions {
    pub cpu_count: usize,
    pub runtime_version: String,
    pub user_agent: String,
    pub seed: String,
    pub permissions: Permissions,
}

impl Default for BootstrapOptions {
    fn default() -> Self {
        let cpu_count = thread::available_parallelism().map(|p| p.get()).unwrap_or(1);
        let runtime_version = env!("CARGO_PKG_VERSION").into();
        let user_agent = format!("JustRuntime/{}", runtime_version);

        Self {
            runtime_version,
            user_agent,
            cpu_count,
            seed: String::from(env!("GIT_HASH_FULL")),
            permissions: Permissions {
                allow_env: permissions::env(),
                allow_net: permissions::net(),
                allow_read: permissions::read(),
                allow_write: permissions::write(),
                allow_cmd: permissions::cmd(),
                allow_sys: permissions::sys(),
            },
        }
    }
}

impl BootstrapOptions {
    pub fn as_json(&self) -> String {
        let payload = json!({
          "seed": self.seed,
          "cpuCount": self.cpu_count,
          "justVersion": self.runtime_version,
          "pid": std::process::id(),
          "target": env!("TARGET"),
          "v8Version": engine::v8_version(),
          "userAgent": self.user_agent,
          "permissions": self.permissions,
        });
        serde_json::to_string_pretty(&payload).unwrap()
    }
}

fn extensions() -> Extension {
    return Extension::builder()
        .js(include_js_files!(
          prefix "[exec:runtime]",
          "core/helpers.js",
          "core/init.js",
          "mod.js",
          "src/core.js",
          "src/process.js",
          "src/log.js",
          "src/go.js",
          "src/format.js",
        ))
        .ops(ops::fs::init())
        .ops(ops::go::init())
        .ops(ops::db::init())
        .ops(ops::sys::init())
        .ops(ops::net::init())
        .ops(ops::core::init())
        .ops(ops::serve::init())
        .build();
}

pub fn import_lib(lib_name: &str) -> &str {
    return match lib_name {
        "io" => include_str!("lib/io.js"),
        "sys" => include_str!("lib/sys.js"),
        "net" => include_str!("lib/net.js"),
        "crypto" => include_str!("lib/crypto/index.js"),
        "crypto:enc" => include_str!("lib/crypto/enc.js"),
        "db:kv" => include_str!("lib/db/kv.js"),
        "db:sqlite" => include_str!("lib/db/sqlite.js"),
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
