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
    let mut args = arg::parse_arguments();
    execute("git diff --stat");
    println!("{:?}", args);
    let mut options: HashMap<String, Opt> = HashMap::new();
    for arg in args {
        options.insert(String::from(arg.long), Opt {
            flag: arg.flag,
            value: arg.value
        });
    }
    execute("git reset");
    if options["commit"].flag {
        execute("git commit -a");
    }
    // if options["push"].flag {
    //     execute("git push");
    // }
}
