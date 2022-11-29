use deno_core::error::AnyError;
use deno_core::op;
use std::fs;

#[op]
pub async fn op_read_file(path: String) -> Result<String, AnyError> {
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
pub async fn op_write_file(path: String, contents: String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op]
pub fn op_remove_file(path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}
