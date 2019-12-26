use crate::{
    colors::{R, X},
    proc::execute_mute,
};
use std::process::{exit, Command};

pub fn invalid_argument(arg: &str) {
    Command::new("sh")
        .arg("-c")
        .arg("echo \"\\07\"")
        .spawn()
        .expect("Failed to execute");

    println!("{}>> Invalid argument: {}{}", R, arg, X);
    exit(1);
}
