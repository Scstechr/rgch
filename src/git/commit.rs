use crate::{
    git::branch,
    git::reset::reset,
    git::status::{check_status, short_status},
    misc::{input, warning},
    proc::execute,
    Opt,
};
use std::collections::HashMap;

pub fn amend() {
    execute(&"git commit --amend");
}

pub fn check_raw_commit<S: ::std::hash::BuildHasher + Default>(args: &HashMap<String, Opt, S>) {
    let branch = branch::get_branch();
    if branch == "master" && args["no-raw"].flag {
        warning("Raw commit not allowed in master branch");
        reset();
    } else {
        commit(&args["commit"].value);
    }
}

pub fn commit(msg: &str) {
    if check_status() {
        short_status();
        let command = if !msg.is_empty() {
            format!("git commit -m \"{}\"", msg)
        } else {
            let q = "Enter commit message";
            let s = input(&q);
            if s != "" {
                format!("git commit -m \"{}\"", s)
            } else {
                "git commit -a".to_string()
            }
        };
        execute(&command);
    }
}
