use crate::ansi::colors::{G, X};
use crate::proc::{execute, execute_out};

pub fn short_status() {
    execute("git status --short");
}

pub fn status() -> bool {
    let output = execute_out("git status --short");
    if !output.is_empty() {
        true
    } else {
        println!("{}Status clean.{}", G, X);
        false
    }
}

pub fn check_status() -> bool {
    let output = execute_out("git status --short");
    if !output.is_empty() {
        true
    } else {
        println!("{}Status clean.{}", G, X);
        false
    }
}
