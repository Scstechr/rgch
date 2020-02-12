use crate::{
    ansi::{
        colors::{G, R, U, X},
        others::ARS,
    },
    error::unimplemented,
    git::{checkout::checkout_new_branch, init::init},
    misc::{beep, confirm, exit_msg, warning},
    proc::execute_out,
};
use std::path::Path;

fn get_branch() -> String {
    let (output, _) = execute_out("git branch");
    let branches: Vec<&str> = output.split('\n').collect();
    let mut current_branch = "";
    for branch in &branches {
        if branch.contains('*') {
            current_branch = branch;
            break;
        }
    }
    current_branch.replace("* ", "")
}

fn get_branch_list() -> Vec<String> {
    let (output, _) = execute_out("git branch");
    let mut branches: Vec<String> = Vec::new();
    let branches_strs: Vec<&str> = output.split('\n').collect();
    for branch in branches_strs {
        if !branch.is_empty() {
            branches.push(branch.replace("*", "").replace(" ", "").to_string());
        }
    }
    branches
}

fn format_branch(branch: &str) -> String {
    format!("{u}{b}{x}", u = U, b = branch, x = X)
}

fn make_branch(branch: &str) {
    checkout_new_branch(&branch);
}

pub fn set_branch(branch: &str, path: &str) -> String {
    let mut final_branch = branch.to_string();
    // let git_dir_path = format!(".git");
    if !Path::new(&".git").exists() {
        let string = format!("Path `{}` does not have a `.git` directory!", path);
        warning(&string);
        let question = "Initialize".to_string();
        if confirm(&question) {
            init(path);
        } else {
            exit_msg(1);
        }
    }
    let current = get_branch();
    let branches = get_branch_list();
    if !branches.is_empty() {
        if !branches.iter().any(|b| b == branch) {
            beep();
            println!(
                "{r}{a}Branch {b}{r} not found.{x}",
                r = R,
                a = ARS,
                b = format_branch(&branch),
                x = X
            );
            let confirm_string = format!("Make branch {}", branch);
            if confirm(&confirm_string) {
                make_branch(&branch);
            // unimplemented();
            } else {
                final_branch = current;
                println!(
                    "{g}{a}Branch set to {b}{g}.{x}",
                    g = G,
                    a = ARS,
                    b = format_branch(&final_branch),
                    x = X
                );
            };
        } else if current != branch {
            beep();
            println!(
                "{r}{a}Currently on {c}{r}, but {b}{r} was chosen.{x}",
                r = R,
                a = ARS,
                c = format_branch(&current),
                b = format_branch(&branch),
                x = X
            );
            let confirm_string = format!("Checkout to branch {b}", b = format_branch(&branch));
            if confirm(&confirm_string) {
                unimplemented();
            } else {
                final_branch = current;
                println!(
                    "{g}{a}Branch set to {b}{g}.{x}",
                    g = G,
                    a = ARS,
                    b = format_branch(&final_branch),
                    x = X
                );
            }
        }
    }
    final_branch
}
