use crate::fn_name;
use crate::state;
use crate::state::Permissions;
use crate::state_err;

use engine::op;
use std::{env, fs};

#[op]
fn env_local(env_name: String) -> String {
    if Permissions::allow_env() {
        let mut path = env::current_dir().unwrap();

        if env_name != "env=find" {
            path.push(&env_name.split("=").collect::<Vec<&str>>()[1]);
            return fs::read_to_string(path).unwrap();
        } else {
            path.push(".env.dev");
            if path.is_file() {
                return fs::read_to_string(path).unwrap();
            } else {
                path.pop();
                path.push(".env.prod");
                if path.is_file() {
                    return fs::read_to_string(path).unwrap();
                } else {
                    path.pop();
                    path.push(".env");
                    if path.is_file() {
                        return fs::read_to_string(path).unwrap();
                    } else {
                        return String::from("");
                    }
                }
            }
        }
    } else {
        return String::from("");
    }
}

#[op]
fn env_get(var: String) -> String {
    if Permissions::allow_env() {
        return match env::var(var) {
            Ok(val) => val,
            Err(_e) => "".to_string(),
        };
    } else {
        return String::from("");
    }
}

#[op]
fn env_set(key: String, var: String) {
    state_err!(Permissions::allow_env(), state::error_env(fn_name!()));
    env::set_var(key, var);
}
