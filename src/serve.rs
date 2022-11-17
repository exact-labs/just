use colored::Colorize;
use deno_core::error::AnyError;
use deno_core::op;
use iron::Iron;
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use std::process::exit;
use std::time::Instant;

#[op]
pub fn op_static(port: u64, dir: String) {
    let path: &Path = Path::new(&dir);

    if !path.exists() {
        println!("Path {:?} does not exist.", path);
        exit(1)
    }
    if !path.is_dir() {
        println!("Path {:?} is not a directory.", path);
        exit(1)
    }
    let mut mount: Mount = Mount::new();
    mount.mount("/", Static::new(path));

    match Iron::new(mount).http(format!("0.0.0.0:{}", port)) {
        Ok(_) => {
            println!("serving path {:?} on :{}", path, port);
        }
        Err(err) => {
            println!("{}", err);
            exit(1)
        }
    }
}

async fn test_url(url: String) -> Result<String, AnyError> {
    println!("testing path {url}");
    print!("result: ");
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}

#[op]
pub async fn op_static_test(port: u64, dir: String) {
    let path: &Path = Path::new(&dir);
    let start = Instant::now();

    if !path.exists() {
        println!("Path {:?} does not exist.", path);
        exit(1)
    }
    if !path.is_dir() {
        println!("Path {:?} is not a directory.", path);
        exit(1)
    }
    let mut mount: Mount = Mount::new();
    mount.mount("/", Static::new(path));

    match Iron::new(mount).http(format!("0.0.0.0:{}", port)) {
        Ok(_) => {
            println!("serving path {:?} on :{}", path, port);
            println!(
                "{:?}",
                test_url(format!("http://localhost:{}", port))
                    .await
                    .unwrap()
            );
            println!(
                "\n{} took {}",
                format!("serve.rs").white(),
                format!("{:.2?}", start.elapsed()).yellow()
            );
            exit(1)
        }
        Err(err) => {
            println!("{}", err);
            exit(1)
        }
    }
}
