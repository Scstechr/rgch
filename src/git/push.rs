use crate::ansi::{colors::X, others::ARS};
use crate::git::pull;
use crate::proc::execute;

pub fn push(remote: &str, branch: &str) {
    // let string = format!("{a}Pull before push{x}", a = ARS, x = X);
    // println!("{}", string);
    pull::pull(remote, branch, false);
    let string = format!("git push {} {}", remote, branch);
    execute(&string);
}
