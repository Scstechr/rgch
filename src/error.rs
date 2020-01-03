use crate::misc::{exit_msg, warning};

pub fn invalid_argument(arg: &str) {
    let string = format!("Invalid argument: {}", arg);
    warning(&string);
    exit_msg(1);
}

pub fn unimplemented() {
    warning("Uninmplemented feature.");
    exit_msg(1);
}

pub fn invalid_path(path: &str) {
    let string = format!("Invalid path: {}", path);
    warning(&string);
    exit_msg(1);
}
