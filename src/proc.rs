use std::collections::HashMap;
use std::process::Command;

use crate::arg;
use crate::colors::{C, X};
use crate::git::commit::commit;
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
    if options["commit"].flag {
        commit();
    }
    if options["push"].flag {
        execute("git push origin master");
    }
}
