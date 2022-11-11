use deno_core::op;
use iron::Iron;
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use std::process::exit;

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
