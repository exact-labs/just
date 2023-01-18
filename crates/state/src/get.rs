use crate::error;
use crate::permissions;

pub fn env(function_path: &str) {
    error!(permissions::env(), error::env(function_path));
}
pub fn net(function_path: &str) {
    error!(permissions::net(), error::net(function_path));
}
pub fn cmd(function_path: &str) {
    error!(permissions::cmd(), error::cmd(function_path));
}
pub fn sys(function_path: &str) {
    error!(permissions::sys(), error::sys(function_path));
}
pub fn read(function_path: &str) {
    error!(permissions::read(), error::read(function_path));
}
pub fn write(function_path: &str) {
    error!(permissions::write(), error::write(function_path));
}
