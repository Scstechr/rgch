use std::collections::HashMap;
use std::process::Command;

use crate::colors::{CYAN, R};
use crate::{Opt};
use crate::arg;

pub fn execute(command: &str) {
    println!("{}>> execute: {}{}", CYAN, command, R);
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to execute");

    let ecode = child.wait().expect("Failed to wait on child");

    assert!(ecode.success());
}

pub fn run() {
//     let _config = arg::parse_defaults();
    let args = arg::parse_arguments();
    println!("{:?}", args);
    let mut options: HashMap<String, Opt> = HashMap::new();
}
