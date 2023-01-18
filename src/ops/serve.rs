use crate::{state, state::Permissions};

use engine::{op, OpDecl};
use macros::function_path;
use std::net::SocketAddr;
use warp::{http::Response, Filter};

pub fn init() -> Vec<OpDecl> {
    vec![serve_directory::decl(), serve_string::decl()]
}

#[op]
async fn serve_directory(host: String, port: i32, path: String) {
    state::error!(Permissions::allow_net(), state::error_net(function_path!()));
    state::error!(Permissions::allow_read(), state::error_read(function_path!()));
    let addr: SocketAddr = format!("{host}:{}", port).parse().expect("Invalid server address");
    println!("serving path '{}' on http://{:?}", path, addr);

    warp::serve(warp::fs::dir(path)).run(addr).await;
}

#[op]
async fn serve_string(host: String, port: i32, string: String, content_type: String) {
    state::error!(Permissions::allow_net(), state::error_net(function_path!()));
    let addr: SocketAddr = format!("{host}:{}", port).parse().expect("Invalid server address");
    let route = warp::any().map(move || Response::builder().header("Content-Type", content_type.clone()).body(string.clone()));
    println!("serving on http://{:?}", addr);

    warp::serve(route).run(addr).await;
}
