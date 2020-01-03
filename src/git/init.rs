use crate::{
    misc::{confirm, exit_msg, warning},
    proc::execute,
};
use std::path::Path;

pub fn init(path: &str) {
    let git_dir_path = format!("{}/.git", path);
    if Path::new(&git_dir_path).exists() {
        let string = format!("Path `{}` already has a `.git` directory!", path);
        warning(&string);
        if confirm("Reinitialize") {
            execute("git init");
        } else {
            exit_msg(1);
        }
    } else {
        execute("git init");
    }
}
