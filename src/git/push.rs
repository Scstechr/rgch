use crate::{
    ansi::{
        colors::{G, X},
        others::ARS,
    },
    git::pull,
    proc::execute,
};

pub fn push(remote: &str, branch: &str) {
    println!("\n{c}{a}Pull before pushing...{x}", a = ARS, c = G, x = X);
    // let string = format!("{a}Pull before push{x}", a = ARS, x = X);
    // println!("{}", string);
    pull::pull(remote, branch, false);
    let string = format!("git push {} {}", remote, branch);
    execute(&string);
}
