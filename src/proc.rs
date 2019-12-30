use std::process::Command;

use crate::ansi::{
    colors::{FC, X},
    others::ARS,
};

pub fn execute(command: &str) -> bool {
    println!("{c}{a}Execute: {v}{x}", c = FC, a = ARS, v = command, x = X);
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

pub fn execute_out(command: &str) -> (String, i32) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    let ecode = output.status.code().unwrap();
    if output.status.success() {
        (
            std::str::from_utf8(&output.stdout).unwrap().to_string(),
            ecode,
        )
    } else {
        (
            std::str::from_utf8(&output.stderr).unwrap().to_string(),
            ecode,
        )
    }
    // println!("{}", output.status);
    // println!("{:?}", output);
    // println!("{:?}", output.stdout);
    // std::str::from_utf8(&output.stdout).unwrap().to_string()
}
