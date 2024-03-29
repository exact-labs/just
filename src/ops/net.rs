use engine::{op, OpDecl};
use macros::{function_name, str};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;

pub fn init() -> Vec<OpDecl> {
    vec![net_get::decl(), net_post::decl()]
}

fn header_parse(headers: String) -> Result<HeaderMap, anyhow::Error> {
    let mut header_list = HeaderMap::new();
    let headers_parsed: Value = serde_json::from_str(&headers)?;
    let parse_header = |val: &str| HeaderValue::from_str(&val[1..val.len() - 1]).unwrap();

    for (key, value) in headers_parsed.as_object().unwrap() {
        header_list.insert(str!(key.to_string()), parse_header(&value.to_string()));
    }

    Ok(header_list)
}

#[op]
async fn net_get(url: String, headers: String) -> Result<String, anyhow::Error> {
    state::get::net(function_name!());
    let client = reqwest::Client::new();
    Ok(client.get(url).headers(header_parse(headers).unwrap()).send().await?.text().await?)
}

#[op]
async fn net_post(url: String, body: String, headers: String) -> Result<String, anyhow::Error> {
    state::get::net(function_name!());
    let client = reqwest::Client::new();
    Ok(client.post(url).headers(header_parse(headers).unwrap()).body(body).send().await?.text().await?)
}
