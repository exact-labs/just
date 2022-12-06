use crate::logger;
use serde::Deserialize;
use std::fs;
use tracing_chrome::{ChromeLayerBuilder, FlushGuard};
use tracing_subscriber::{filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Layer};

#[derive(Debug, Deserialize)]
pub struct Project {
    pub index: String,
}

pub fn read_index(dir: std::path::Display, package: &String, version: &String) -> Project {
    let contents = match fs::read_to_string(format!("{dir}/packages/{package}/{version}/package.yml")) {
        Ok(text) => text,
        Err(_) => {
            logger::error(format!("{package}@{version} not found. Did you run 'just install'"));
            std::process::exit(1);
        }
    };

    let yaml_file: Result<Project, _> = serde_yaml::from_str(&contents);

    let parsed = match yaml_file {
        Ok(project) => project,
        Err(error) => {
            logger::error(format!("{} in package.yml", error));
            std::process::exit(1);
        }
    };

    return parsed;
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn trim_start_end(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

pub fn init_trace(out_file: &Option<String>) -> Option<FlushGuard> {
    let mut layer = ChromeLayerBuilder::new().include_args(true);

    if let Some(trace_out_file) = out_file {
        layer = layer.file(trace_out_file.clone());
    }

    let (chrome_layer, guard) = layer.build();
    tracing_subscriber::registry()
        .with(chrome_layer.with_filter(filter::filter_fn(|metadata| !metadata.target().contains("cranelift") && !metadata.name().contains("log "))))
        .try_init()
        .expect("Should able to register trace");

    Some(guard)
}
