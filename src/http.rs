use deno_core::error::AnyError;
use deno_core::op;

#[op]
pub async fn op_fetch(url: String) -> Result<String, AnyError> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}
