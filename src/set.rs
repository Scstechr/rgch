use crate::error::invalid_path;
use std::{env, path::Path};

pub fn set_git_dir(path: &str) -> bool {
    let path = Path::new(path);
    if env::set_current_dir(&path).is_ok() {
        true
    } else {
        let path = path.to_str().unwrap();
        invalid_path(path);
        false
    }
}
