use crate::ansi::colors::{G, X};
use crate::proc::execute_out;

pub fn status() -> bool {
    let output = execute_out("git status --short");
    if !output.is_empty() {
        true
    } else {
        println!("{}Status clean.{}", G, X);
        false
    }
}
