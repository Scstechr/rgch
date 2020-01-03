extern crate serde;
extern crate toml;
use crate::{error::invalid_path, Opt};
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

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

pub fn set_default<S: ::std::hash::BuildHasher>(args: &HashMap<String, Opt, S>) {
    for arg in args {
        println!("{:?}", arg);
    }
    let mut file = File::create(".config.toml").unwrap();
    let toml = toml::to_string(&args).unwrap();
    write!(file, "{}", toml).unwrap();
    file.flush().unwrap();
    println!("\n#TOML:\n{}", toml);
}
