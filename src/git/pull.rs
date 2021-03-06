use crate::{
    ansi::{
        colors::{G, X},
        others::ARS,
    },
    proc::execute,
};
use std::process::exit;

pub fn pull(given_remote: &str, given_branch: &str, exit_flag: bool) {
    // println!("{}", given_input);
    println!(
        "\n{c}{a}Pull (fetch, merge) from remote repository...{x}",
        a = ARS,
        c = G,
        x = X
    );
    let remote = &given_remote;
    let branch = &given_branch;
    // let url = set_clone_url(&given_url);
    // let branch = set_clone_branch(&given_branch, given_input);
    // let name = set_clone_dir(&url);
    let command = format!("git fetch {} {}", remote, branch);
    execute(&command);
    let command = format!("git merge {}/{}", remote, branch);
    execute(&command);
    if exit_flag {
        exit(0);
    }
}
