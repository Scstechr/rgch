use crate::{
    git::status::{check_status, short_status},
    misc::input,
    proc::execute,
    Opt,
};
use std::collections::HashMap;

pub fn amend() {
    execute(&"git commit --amend");
}

pub fn check_raw<S: ::std::hash::BuildHasher + Default>(args: &HashMap<String, Opt, S>) -> bool {
    args["branch"].value == "master" && !args["no-raw"].flag
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
