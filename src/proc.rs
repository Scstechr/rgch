use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::process::Command;

use crate::arg;
use crate::colors::{C, G, R};
use crate::Opt;

fn status() -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg("git status --short")
        .output()
        .expect("failed to execute process");
    if output.stdout.len() > 0 {
        true
    } else {
        println!("{}Status clean.{}", G, R);
        false
    }
}

pub fn execute(command: &str) -> bool {
    println!("{}>> execute: {}{}", C, command, R);
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
    let args = arg::parse_arguments();
    execute("git diff --stat");
    // println!("{:?}", args);
    let mut options: HashMap<String, Opt> = HashMap::new();
    for arg in args {
        options.insert(
            String::from(arg.long),
            Opt {
                flag: arg.flag,
                value: arg.value,
            },
        );
    }
    if options["commit"].flag && status() {
        print!("Enter commit message: ");
        let mut s = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("-a");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        let command = if s != "" {
            format!("git commit -a -m \"{}\"", s)
        } else {
            format!("git commit -a")
        };
        execute(&command);
    }
    if options["push"].flag {
        execute("git push origin master");
    }
}
