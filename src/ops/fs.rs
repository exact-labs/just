use crate::helpers;
use engine::op;
use std::{fs, path::PathBuf};

#[op]
pub async fn op_read_file(path: String) -> Result<String, anyhow::Error> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op]
pub fn op_read_dir(path: String) -> Vec<String> {
    let mut vec = Vec::new();
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        vec.push(format!("{}", path.unwrap().path().display()));
    }

    return vec;
}

#[op]
pub async fn op_write_file(path: String, contents: String) -> Result<(), anyhow::Error> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op]
pub async fn op_make_dir(path: String) -> Result<(), anyhow::Error> {
    tokio::fs::create_dir(path).await?;
    Ok(())
}

#[op]
pub async fn op_remove_file(path: String) -> Result<(), anyhow::Error> {
    tokio::fs::remove_file(path).await?;
    Ok(())
}

#[op]
pub async fn op_remove_dir(path: String) -> Result<(), anyhow::Error> {
    tokio::fs::remove_dir(path).await?;
    Ok(())
}

#[op]
pub fn op_file_sha(path: String) -> Result<String, anyhow::Error> {
    let file_sha = helpers::sha256_digest(&PathBuf::from(path.clone()))?;
    Ok(file_sha)
}
