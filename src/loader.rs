use crate::helpers;
use crate::runtime;

use anyhow::{anyhow, bail, Context};
use ast::{parse_module, MediaType, ParseParams, SourceTextInfo};
use colored::Colorize;
use data_url::DataUrl;
use engine::{futures::FutureExt, ModuleLoader, ModuleSource, ModuleSourceFuture, ModuleSpecifier, ModuleType};
use macros::{str, ternary};
use std::{error::Error, fmt, path::Component, path::Path, path::PathBuf, pin::Pin};
use url::{ParseError, Url};
use ModuleResolutionError::*;

pub struct RuntimeImport;
pub const DUMMY_SPECIFIER: &str = "<unknown>";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModuleResolutionError {
    InvalidUrl(ParseError),
    InvalidBaseUrl(ParseError),
    InvalidPath(PathBuf),
    ImportPrefixMissing(String, Option<String>),
}

impl Error for ModuleResolutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InvalidUrl(ref err) | InvalidBaseUrl(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for ModuleResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvalidUrl(ref err) => write!(f, "invalid URL: {}", err),
            InvalidBaseUrl(ref err) => {
                write!(f, "invalid base URL for relative import: {}", err)
            }
            InvalidPath(ref path) => write!(f, "invalid module path: {:?}", path),
            ImportPrefixMissing(ref specifier, ref maybe_referrer) => write!(
                f,
                "Relative import path \"{}\" not prefixed with / or ./ or ../{}",
                specifier,
                match maybe_referrer {
                    Some(referrer) => format!(" from \"{}\"", referrer),
                    None => String::new(),
                }
            ),
        }
    }
}

pub fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut components = path.as_ref().components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}

pub fn resolve_import(specifier: &str, base: &str) -> Result<ModuleSpecifier, ModuleResolutionError> {
    let url = match Url::parse(specifier) {
        Ok(url) => url,
        Err(ParseError::RelativeUrlWithoutBase) if !(specifier.starts_with("just/") || specifier.starts_with('/') || specifier.starts_with("./") || specifier.starts_with("../")) => {
            let maybe_referrer = if base.is_empty() { None } else { Some(base.to_string()) };
            return Err(ImportPrefixMissing(specifier.to_string(), maybe_referrer));
        }
        Err(ParseError::RelativeUrlWithoutBase) => {
            let base = if base == DUMMY_SPECIFIER {
                let path = std::env::current_dir().unwrap().join(base);
                Url::from_file_path(path).unwrap()
            } else {
                Url::parse(base).map_err(InvalidBaseUrl)?
            };
            base.join(specifier).map_err(InvalidUrl)?
        }
        Err(err) => return Err(InvalidUrl(err)),
    };

    Ok(url)
}

pub fn specifier_has_uri_scheme(specifier: &str) -> bool {
    let mut chars = specifier.chars();
    let mut len = 0usize;
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() => len += 1,
        _ => return false,
    }

    loop {
        match chars.next() {
            Some(c) if c.is_ascii_alphanumeric() || "+-.".contains(c) => len += 1,
            Some(':') if len >= 2 => return true,
            _ => return false,
        }
    }
}

pub fn resolve_url(url_str: &str) -> Result<Url, ModuleResolutionError> {
    Url::parse(url_str).map_err(ModuleResolutionError::InvalidUrl)
}

pub fn resolve_path(path_str: &str) -> Result<Url, ModuleResolutionError> {
    let path = std::env::current_dir().map_err(|_| ModuleResolutionError::InvalidPath(path_str.into()))?.join(path_str);
    let path = normalize_path(&path);
    Url::from_file_path(path.clone()).map_err(|()| ModuleResolutionError::InvalidPath(path))
}

pub fn import_prefix(specifier: &str) -> Result<Url, ModuleResolutionError> {
    ternary!(specifier_has_uri_scheme(specifier), resolve_url(specifier), resolve_path(specifier))
}

impl ModuleLoader for RuntimeImport {
    fn resolve(&self, specifier: &str, referrer: &str, _is_main: bool) -> Result<ModuleSpecifier, anyhow::Error> {
        Ok(resolve_import(specifier, referrer)?)
    }

    fn load(&self, module_specifier: &ModuleSpecifier, _maybe_referrer: Option<ModuleSpecifier>, _is_dyn_import: bool) -> Pin<Box<ModuleSourceFuture>> {
        let string_specifier = module_specifier.to_string();
        let module_specifier = module_specifier.clone();

        async move {
            let module_array = module_specifier.path().split("/").collect::<Vec<&str>>();
            let module_prefix = module_array[module_array.len() - 2];
            let module_name = module_array.last().context("Unable to find module name.")?;
            let env_args = std::env::var("_just_args")?;

            let (bytes, media_type, module_type, should_transpile) = match module_specifier.scheme() {
                "http" | "https" => {
                    let home_dir = helpers::get_home_dir()?;
                    let folder_exists: bool = helpers::Exists::folder(format!("{}/.just/packages", home_dir.display()))?;

                    let package_directory: String = if module_specifier.path().ends_with(".js")
                        || module_specifier.path().ends_with(".ts")
                        || module_specifier.path().ends_with(".jsx")
                        || module_specifier.path().ends_with(".tsx")
                        || module_specifier.path().ends_with(".cjs")
                        || module_specifier.path().ends_with(".mjs")
                        || module_specifier.path().ends_with("json")
                    {
                        format!("{}/.just/packages/{}{}", &home_dir.display(), module_specifier.domain().unwrap(), module_specifier.path())
                    } else {
                        format!("{}/.just/packages/{}{}/mod.js", &home_dir.display(), module_specifier.domain().unwrap(), module_specifier.path(),)
                    };

                    if !folder_exists {
                        std::fs::create_dir_all(format!("{}/.just/packages", &home_dir.display())).unwrap();
                        println!("created {}/.just/packages", &home_dir.display());
                    }

                    if module_prefix != "just" && (!helpers::Exists::file(package_directory.clone())? || env_args.contains("cache=skip")) {
                        let res = reqwest::get(module_specifier.clone()).await?;
                        let res = res.error_for_status()?;
                        let content_type = res.headers().get("Content-Type").unwrap().to_str().unwrap();
                        let download_path = format!("{}/.just/packages/{}{}", &home_dir.display(), res.url().host().unwrap(), res.url().path());

                        println!("{} {}", "download".green(), res.url());

                        log::debug!("content_type: {}", content_type);
                        log::debug!("download_path: {}", download_path);
                        log::debug!("package_directory: {}", package_directory);
                        log::debug!("module_specifier: {}", module_specifier);

                        if content_type.contains(&"text/plain")
                            || content_type.contains(&"text/jsx")
                            || content_type.contains(&"text/tsx")
                            || content_type.contains(&"text/node")
                            || content_type.contains(&"text/json")
                            || content_type.contains(&"text/jscript")
                            || content_type.contains(&"text/javascript")
                            || content_type.contains(&"text/typescript")
                            || content_type.contains(&"text/ecmascript")
                            || content_type.contains(&"application/jsx")
                            || content_type.contains(&"application/tsx")
                            || content_type.contains(&"application/node")
                            || content_type.contains(&"application/json")
                            || content_type.contains(&"text/x-javascript")
                            || content_type.contains(&"text/x-typescript")
                            || content_type.contains(&"application/jscript")
                            || content_type.contains(&"application/javascript")
                            || content_type.contains(&"application/typescript")
                            || content_type.contains(&"application/ecmascript")
                            || content_type.contains(&"application/x-javascript")
                            || content_type.contains(&"application/x-typescript")
                        {
                            let mut trimmed_path = package_directory.split("/").collect::<Vec<&str>>();
                            trimmed_path.pop();

                            log::debug!("trimmed_path: {}", trimmed_path.join("/"));
                            tokio::fs::create_dir_all(trimmed_path.join("/")).await?;
                            tokio::fs::write(&package_directory, res.bytes().await?).await?;
                        } else {
                            tokio::fs::create_dir_all(&download_path).await?;
                        }
                    }

                    let bytes = ternary!(module_prefix == "just", runtime::import_lib(module_name).into(), tokio::fs::read(&package_directory).await?);
                    let (module_type, should_transpile) = if module_prefix == "just" {
                        (ModuleType::JavaScript, false)
                    } else {
                        match MediaType::from(Path::new(&package_directory)) {
                            MediaType::TypeScript | MediaType::Mts | MediaType::Cts | MediaType::Dts | MediaType::Dmts | MediaType::Dcts | MediaType::Tsx => (ModuleType::JavaScript, true),
                            MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => (ModuleType::JavaScript, false),
                            MediaType::Jsx => (ModuleType::JavaScript, true),
                            MediaType::Json => (ModuleType::Json, false),
                            _ => bail!("Unknown file extension {:?}", Path::new(&package_directory).extension()),
                        }
                    };

                    (bytes, MediaType::from(&package_directory), module_type, should_transpile)
                }
                "file" => {
                    let path = module_specifier.to_file_path().map_err(|_| anyhow!("Only file: URLs are supported."))?;
                    let bytes = ternary!(module_prefix == "just", runtime::import_lib(module_name).into(), tokio::fs::read(&path).await?);

                    let (module_type, should_transpile) = if module_prefix == "just" {
                        (ModuleType::JavaScript, false)
                    } else {
                        match MediaType::from(&path) {
                            MediaType::TypeScript | MediaType::Mts | MediaType::Cts | MediaType::Dts | MediaType::Dmts | MediaType::Dcts | MediaType::Tsx => (ModuleType::JavaScript, true),
                            MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => (ModuleType::JavaScript, false),
                            MediaType::Jsx => (ModuleType::JavaScript, true),
                            MediaType::Json => (ModuleType::Json, false),
                            _ => bail!("Unknown file extension {:?}", path.extension()),
                        }
                    };

                    (bytes, MediaType::from(&path), module_type, should_transpile)
                }
                "data" => {
                    let data_url = DataUrl::process(module_specifier.as_str()).map_err(|e| anyhow!("Unable to parse data url {:?}.", e))?;
                    let (bytes, _) = data_url.decode_to_vec().map_err(|e| anyhow!("Unable to parse data url {:?}.", e))?;
                    let mime_type = data_url.mime_type().subtype.clone();

                    let (module_type, should_transpile) = match str!(mime_type.clone()) {
                        "javascript" | "ecmascript" | "x-javascript" | "node" => (ModuleType::JavaScript, false),
                        "typescript" | "x-typescript" | "tsx" => (ModuleType::JavaScript, true),
                        "jsx" | "jscript" => (ModuleType::JavaScript, true),
                        "json" => (ModuleType::Json, false),
                        _ => bail!("Unknown mime type {:?}", data_url.mime_type().subtype),
                    };

                    (bytes, MediaType::from_content_type(&module_specifier, &mime_type), module_type, should_transpile)
                }
                schema => bail!("Invalid schema {}", schema),
            };

            let code = if should_transpile {
                let parsed = parse_module(ParseParams {
                    specifier: string_specifier.clone(),
                    text_info: SourceTextInfo::from_string(String::from_utf8_lossy(&bytes).into_owned()),
                    media_type,
                    capture_tokens: false,
                    scope_analysis: false,
                    maybe_syntax: None,
                })?;
                parsed.transpile(&Default::default())?.text
            } else {
                String::from_utf8_lossy(&bytes).into_owned()
            };

            let module = ModuleSource {
                code: code.into_bytes().into_boxed_slice(),
                module_type,
                module_url_specified: string_specifier.clone(),
                module_url_found: string_specifier,
            };

            Ok(module)
        }
        .boxed_local()
    }
}
