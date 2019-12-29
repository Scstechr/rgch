use crate::ansi::{
    arrows::RET,
    colors::{F, G, X},
};
use crate::misc::{confirm, input};
use crate::proc::execute;

use std::process::exit;

fn set_clone_url(given: &str) -> String {
    let url = if given != "None" {
        given.to_string()
    } else {
        input("Enter URL")
    };
    println!("{}   {} Remote repository set to: {}{}", F, RET, X, url);
    url.replace("@", "https://github.com/")
}

pub fn clone(given_url: &str, given_branch: &str, given_input: bool) {
    // println!("{}", given_input);
    println!("{}>> Cloning remote repository...{}", G, X);
    // println!("{}, {}", given_url, given_branch);
    let url = set_clone_url(&given_url);
    let branch = if given_input {
        given_branch.to_string()
    } else if confirm("Is it master branch") {
        "master".to_string()
    } else {
        let b = input("Enter branch name");
        if !b.is_empty() {
            b
        } else {
            "master".to_string()
        }
    };
    println!("{}   {} Remote branch set to: {}{}", F, RET, X, branch);
    let name: Vec<&str> = url.split('/').collect();
    let name = name.last().unwrap();
    println!(
        "{}   {} Cloning remote repository to: {}{}",
        F, RET, X, name
    );
    let command = format!("git clone -b {} {} {}", branch, url, name);
    execute(&command);
    exit(0);
}
