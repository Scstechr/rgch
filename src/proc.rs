use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::str::FromStr;

use crate::arg;
use crate::colors::{CYAN, R};
use crate::Opt;

fn status() -> bool {
    let output = Command::new("sh")
            .arg("-c")
            .arg("git status --short")
            .output()
            .expect("failed to execute process");
    let out = output.stdout;
    println!("{:?}, {}", out, out.len());
    true
}

pub fn execute(command: &str) -> bool{
    println!("{}>> execute: {}{}", CYAN, command, R);
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to execute");

    let ecode = child.wait().expect("Failed to wait on child");

    ecode.success()
    // assert!(ecode.success());
}

pub fn run() {
    //     let _config = arg::parse_defaults();
    let mut args = arg::parse_arguments();
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
    execute("git reset");
    println!("{}", status());
    // if options["commit"].flag {
    //     print!("Enter commit message: ");
    //     let mut s = String::new();
    //     let _ = stdout().flush();
    //     stdin().read_line(&mut s).expect("-a");
    //     if let Some('\n') = s.chars().next_back() {
    //         s.pop();
    //     }
    //     if let Some('\r') = s.chars().next_back() {
    //         s.pop();
    //     }
    //     let command = if s == "-a" {
    //         format!("git commit {}", s)
    //     } else {
    //         format!("git commit -a -m \"{}\"", s)
    //     };
    //     execute(&command);
    //     // let string = read<String>();
    //     // println!("{}", string);
    //     //     execute("git commit -a");
    // }
    // if options["push"].flag {
    //     execute("git push origin master");
    // }
}
