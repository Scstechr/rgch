#![allow(dead_code)]
#![allow(unused_variables)]
use crate::ansi::{
    colors::{R, X},
    others::ARS,
};
use crate::git::{format, git_path_check, set_url};
use crate::misc::beep;
use crate::proc::{execute, execute_out};

fn get_remote_list() -> Vec<String> {
    let (output, _) = execute_out("git remote");
    let mut remotes: Vec<String> = Vec::new();
    let remotes_strs: Vec<&str> = output.split('\n').collect();
    for remote in remotes_strs {
        if !remote.is_empty() {
            remotes.push(remote.replace(" ", "").to_string());
        }
    }
    remotes
}

fn add_remote(remote: &str) {
    let url = set_url("");
    let cmd = format!("git remote add {} {}", remote, url);
    execute(&cmd);
}

pub fn set_remote(remote: &str, path: &str) -> String {
    git_path_check(&path);
    let remotes = get_remote_list();
    if !remotes.iter().any(|r| r == remote) {
        beep();
        println!(
            "{r}{a}Remote branch {b}{r} not found.{x}",
            r = R,
            a = ARS,
            b = format(&remote),
            x = X
        );
    }
    println!("{:#?}", get_remote_list());
    "origin".to_string()
}
