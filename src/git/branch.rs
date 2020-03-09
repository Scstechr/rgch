use crate::{
    ansi::{
        colors::{G, R, X},
        others::ARS,
    },
    git::{
        checkout::{checkout, checkout_new_branch},
        format, git_path_check,
    },
    misc::{beep, confirm},
    proc::execute_out,
};

pub fn get_branch() -> String {
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

fn make_branch(branch: &str) {
    checkout_new_branch(&branch);
}

pub fn set_branch(branch: &str, path: &str) -> String {
    let mut final_branch = branch.to_string();
    // let git_dir_path = format!(".git");
    git_path_check(&path);
    let current = get_branch();
    let branches = get_branch_list();
    if !branches.iter().any(|b| b == branch) {
        beep();
        println!(
            "{r}{a}Branch {b}{r} not found.{x}",
            r = R,
            a = ARS,
            b = format(&branch),
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
                b = format(&final_branch),
                x = X
            );
        };
    } else if current != branch {
        beep();
        println!(
            "{r}{a}Currently on {c}{r}, but {b}{r} was chosen.{x}",
            r = R,
            a = ARS,
            c = format(&current),
            b = format(&branch),
            x = X
        );
        let confirm_string = format!("Checkout to branch {b}", b = format(&branch));
        if confirm(&confirm_string) {
            checkout(&branch);
        // unimplemented();
        } else {
            final_branch = current;
            println!(
                "{g}{a}Branch set to {b}{g}.{x}",
                g = G,
                a = ARS,
                b = format(&final_branch),
                x = X
            );
        }
    }
    final_branch
}
