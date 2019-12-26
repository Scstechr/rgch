use crate::colors::{G, X};
use std::process::Command;

pub fn status() -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg("git status --short")
        .output()
        .expect("failed to execute process");
    if !output.stdout.is_empty() {
        true
    } else {
        println!("{}Status clean.{}", G, X);
        false
    }
}
