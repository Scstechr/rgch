use std::process::Command;

use crate::arg::{help, parse_arguments};
use crate::colors::{C, S, X};
use crate::git::{branch::set_branch, commit::commit, diff::diff, push::push};

const VERSION: &str = env!("CARGO_PKG_VERSION");

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

pub fn execute_out(command: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    std::str::from_utf8(&output.stdout).unwrap().to_string()
}

pub fn run() {
    println!("{}rgch v{}: Rust implementation of gch{}", S, VERSION, X);
    //     let _config = arg::parse_defaults();
    let args = parse_arguments();

    // for arg in &args {
    //     println!("{:?}", arg);
    // }

    if args["help"].flag {
        help();
    }

    let branch = set_branch(&args["branch"].value);

    diff(args["verbose"].flag);

    execute_mute("git add .");
    execute("git status");
    execute_mute("git reset");

    if args["commit"].flag {
        commit();
    }
    if args["push"].flag {
        push(&branch);
    }
}
