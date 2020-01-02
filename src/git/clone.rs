use crate::ansi::{
    arrows::RET,
    colors::{F, G, U, X},
    moves::pos_x,
    others::{ARS, TAB},
};
use crate::misc::{confirm, input};
use crate::proc::execute;

use std::process::exit;

const POS_X_ARG: u64 = 31;

fn set_clone_url(given: &str) -> String {
    let url = if given != "None" {
        given.to_string()
    } else {
        input("Enter URL (@ for GitHub)")
    };
    let url = url.replace("@", "https://github.com/");
    println!(
        "{f}{t}{r} Remote repository set to {a}: {x}{u}{v}{x}",
        a = pos_x(POS_X_ARG),
        t = TAB,
        f = F,
        r = RET,
        u = U,
        x = X,
        v = url
    );
    url
}

fn set_clone_branch(given: &str, flag: bool) -> String {
    let branch = if flag {
        given.to_string()
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
    println!(
        "{f}{t}{r} Remote branch set to {a}: {x}{u}{v}{x}",
        a = pos_x(POS_X_ARG),
        t = TAB,
        f = F,
        r = RET,
        u = U,
        x = X,
        v = branch
    );
    branch
}

fn set_clone_dir(url: &str) -> String {
    let name: Vec<&str> = url.split('/').collect();
    let name = name.last().unwrap();
    let question = format!("Clone it to `{}", name);
    let name = if confirm(&question) {
        (*name).to_string()
    } else {
        let b = input("Enter directory name");
        if !b.is_empty() {
            b
        } else {
            (*name).to_string()
        }
    };
    println!(
        "{f}{t}{r} Cloning remote repository to {a}: {x}{u}{v}{x}",
        a = pos_x(POS_X_ARG),
        t = TAB,
        f = F,
        r = RET,
        u = U,
        x = X,
        v = name
    );
    name
}

pub fn clone(given_url: &str, given_branch: &str, given_input: bool) {
    // println!("{}", given_input);
    println!(
        "\n{c}{a}Cloning remote repository...{x}",
        a = ARS,
        c = G,
        x = X
    );
    // println!("{}, {}", given_url, given_branch);
    let url = set_clone_url(&given_url);
    let branch = set_clone_branch(&given_branch, given_input);
    let name = set_clone_dir(&url);
    let command = format!("git clone -b {} {} {}", branch, url, name);
    execute(&command);
    exit(0);
}
