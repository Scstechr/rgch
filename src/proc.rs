use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::process::Command;

use crate::arg;
use crate::colors::{C, X};
use crate::git::status::status;
use crate::Opt;

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
    let args = arg::parse_arguments();
    // execute("git add *");
    execute("git diff --stat");
    execute_mute("git reset");
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
        print!("\nEnter commit message: ");
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
            "git commit -a".to_string()
        };
        execute(&command);
    }
    if options["push"].flag {
        execute("git push origin master");
    }
}
