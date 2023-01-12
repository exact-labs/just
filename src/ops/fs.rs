use crate::helpers;
use engine::{op, OpDecl};
use std::{fs, path::PathBuf};

pub fn init() -> Vec<OpDecl> {
    vec![
        read_file::decl(),
        write_file::decl(),
        remove_file::decl(),
        dir_list::decl(),
        make_dir::decl(),
        remove_dir::decl(),
        file_sha::decl(),
    ]
}

#[op]
fn file_sha(path: String) -> Result<String, anyhow::Error> {
    Ok(helpers::sha256_digest(&PathBuf::from(path.clone()))?)
}

#[op]
async fn read_file(path: String) -> Result<String, anyhow::Error> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op]
async fn write_file(path: String, contents: String) -> Result<(), anyhow::Error> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op]
async fn remove_file(path: String) -> Result<(), anyhow::Error> {
    tokio::fs::remove_file(path).await?;
    Ok(())
}

#[op]
fn dir_list(path: String) -> Vec<String> {
    let mut vec = Vec::new();
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        vec.push(format!("{}", path.unwrap().path().display()));
    }

    return vec;
}

#[op]
async fn make_dir(path: String) -> Result<(), anyhow::Error> {
    tokio::fs::create_dir_all(path).await?;
    Ok(())
}

#[op]
async fn remove_dir(path: String) -> Result<(), anyhow::Error> {
    tokio::fs::remove_dir(path).await?;
    Ok(())
}
