use std::pin::Pin;

use colored::Colorize;
use data_url::DataUrl;
use deno_core::anyhow::bail;
use deno_core::futures::FutureExt;
use deno_core::ModuleLoader;
use deno_core::ModuleSource;
use deno_core::ModuleSourceFuture;
use deno_core::ModuleSpecifier;
use deno_core::ModuleType;
use std::error::Error;
use std::fmt;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use url::ParseError;
use url::Url;
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

pub fn resolve_import(
    specifier: &str,
    base: &str,
) -> Result<ModuleSpecifier, ModuleResolutionError> {
    let url = match Url::parse(specifier) {
        Ok(url) => url,
        Err(ParseError::RelativeUrlWithoutBase)
            if !(specifier.starts_with('/')
                || specifier.starts_with("./")
                || specifier.starts_with("../")) =>
        {
            let maybe_referrer = if base.is_empty() {
                None
            } else {
                Some(base.to_string())
            };
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
    let path = std::env::current_dir()
        .map_err(|_| ModuleResolutionError::InvalidPath(path_str.into()))?
        .join(path_str);
    let path = normalize_path(&path);
    Url::from_file_path(path.clone()).map_err(|()| ModuleResolutionError::InvalidPath(path))
}

pub fn import_prefix(specifier: &str) -> Result<Url, ModuleResolutionError> {
    if specifier_has_uri_scheme(specifier) {
        resolve_url(specifier)
    } else {
        resolve_path(specifier)
    }
}

impl ModuleLoader for RuntimeImport {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _is_main: bool,
    ) -> Result<ModuleSpecifier, deno_core::anyhow::Error> {
        Ok(resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<ModuleSpecifier>,
        _is_dyn_import: bool,
    ) -> Pin<Box<ModuleSourceFuture>> {
        let module_specifier = module_specifier.clone();
        let string_specifier = module_specifier.to_string();

        async {
            let mut module_type = ModuleType::JavaScript;
            let bytes = match module_specifier.scheme() {
                "http" | "https" => {
                    println!("{} {module_specifier}", "download".green());
                    let res = reqwest::get(module_specifier).await?;
                    let res = res.error_for_status()?;
                    res.bytes().await?
                }
                "file" => {
                    let path = match module_specifier.to_file_path() {
                        Ok(path) => path,
                        Err(_) => bail!("Invalid file URL."),
                    };
                    module_type = if let Some(extension) = path.extension() {
                        let ext = extension.to_string_lossy().to_lowercase();
                        if ext == "json" {
                            ModuleType::Json
                        } else {
                            ModuleType::JavaScript
                        }
                    } else {
                        ModuleType::JavaScript
                    };
                    let bytes = tokio::fs::read(path).await?;
                    bytes.into()
                }
                "data" => {
                    let url = match DataUrl::process(module_specifier.as_str()) {
                        Ok(url) => url,
                        Err(_) => bail!("Not a valid data URL."),
                    };
                    let bytes = match url.decode_to_vec() {
                        Ok((bytes, _)) => bytes,
                        Err(_) => bail!("Not a valid data URL."),
                    };
                    bytes.into()
                }
                schema => bail!("Invalid schema {}", schema),
            };

            let bytes = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
                bytes.slice(3..)
            } else {
                bytes
            };

            Ok(ModuleSource {
                code: bytes.to_vec().into_boxed_slice(),
                module_type: module_type,
                module_url_specified: string_specifier.clone(),
                module_url_found: string_specifier,
            })
        }
        .boxed_local()
    }
}
