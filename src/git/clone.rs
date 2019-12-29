use crate::colors::{G, X};
use crate::misc::{confirm, input};
use crate::proc::execute;

use std::process::exit;

pub fn clone(given_url: &str, given_branch: &str, given_input: bool) {
    // println!("{}", given_input);
    println!("{}>> Cloning remote repository{}", G, X);
    // println!("{}, {}", given_url, given_branch);
    let url = if given_url != "None" {
        given_url.to_string()
    } else {
        input("Enter URL")
    };
    println!("{}>> Remote repository set to: {}{}", G, X, url);
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
    println!("{}\n>> Remote branch set to: {}{}", G, X, branch);
    let name: Vec<&str> = url.split('/').collect();
    let name = name.last().unwrap();
    println!("{}\n>> Cloning remote repository to: {}{}", G, X, name);
    let command = format!("git clone -b {} {} {}", branch, url, name);
    execute(&command);
    exit(0);
}
