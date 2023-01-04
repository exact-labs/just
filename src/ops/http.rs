use engine::op;

#[op]
pub async fn op_get(url: String) -> Result<String, anyhow::Error> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}

#[op]
pub async fn op_post(url: String, body: String) -> Result<String, anyhow::Error> {
    let client = reqwest::Client::new();
    let res = client.post(url).body(body).send().await?.text().await?;
    Ok(res)
}
