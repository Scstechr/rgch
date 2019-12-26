use std::process::Command;

use crate::arg::{help, parse_arguments};
use crate::colors::{C, X};
use crate::git::{commit::commit, diff::diff, push::push};

pub fn execute(command: &str) -> bool {
    println!("{}>> execute: {}{}", C, command, X);
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to execute");

    let ecode = child.wait().expect("Failed to wait on child");

    ecode.success()
    // assert!(ecode.success());
}

pub fn execute_mute(command: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    // assert!(ecode.success());
}

pub fn run() {
    //     let _config = arg::parse_defaults();
    let args = parse_arguments();

    if args["help"].flag {
        help();
    }

    diff(args["verbose"].flag);

    if args["commit"].flag {
        commit();
    }
    if args["push"].flag {
        push();
    }
}
