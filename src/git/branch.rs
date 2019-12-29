use crate::{
    ansi::{
        colors::{G, R, U, X},
        others::ARS,
    },
    error::unimplemented,
    misc::{beep, confirm},
    proc::execute_out,
};
use std::process::exit;

fn get_branch() -> String {
    let output = execute_out("git branch");
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
    let output = execute_out("git branch");
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

pub fn set_branch(branch: &str) -> String {
    let mut final_branch = branch.to_string();
    let current = get_branch();
    let branches = get_branch_list();
    if !branches.is_empty() {
        let check = branches.iter().any(|b| b == branch);
        if !check {
            beep();
            println!(
                "{r}{a}Branch {b}{r} not found.{x}",
                r = R,
                a = ARS,
                b = format_branch(&branch),
                x = X
            );
            let confirm_string = format!("Make branch {}?", branch);
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
            };
        } else if current != branch {
            beep();
            println!(
                "{}>> Currently on \"{}\", but \"{}\" was chosen.{}",
                R, current, branch, X
            );
            let confirm_string = format!("Checkout to branch \"{}\"?", branch);
            if confirm(&confirm_string) {
                unimplemented();
            } else {
                final_branch = current;
                println!("{}>> Branch set to \"{}\".{}", G, final_branch, X);
            }
        }
    } else {
        beep();
        println!(
            "{r}{a}Does not have a .git folder{x}",
            a = ARS,
            r = R,
            x = X
        );
        exit(1);
    }
    final_branch
}
