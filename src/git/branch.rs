use crate::{
    colors::{G, R, X},
    misc::{beep, confirm},
    proc::execute_out,
};

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
            branches.push(branch.replace("* ", "").replace("  ", "").to_string());
        }
    }
    branches
}

pub fn set_branch(branch: &str) -> String {
    let mut final_branch = branch.to_string();
    let current = get_branch();
    let branches = get_branch_list();
    let check = branches.iter().any(|b| b == branch);
    // println!(
    //     "current: {}, selected: {}, branches: {:?}",
    //     current, branch, branches
    // );
    if !check {
        beep();
        println!("{}>> Branch \"{}\" not found.{}", R, branch, X);
        let confirm_string = format!("Make branch \"{}\"?", branch);
        if confirm(&confirm_string) {
            unimplemented!();
        } else {
            final_branch = current;
            println!("{}>> Branch set to \"{}\".{}", G, final_branch, X);
        };
    } else if current != branch {
        beep();
        println!(
            "{}>> Currently on \"{}\", but \"{}\" was chosen.{}",
            R, current, branch, X
        );
        let confirm_string = format!("Checkout to branch \"{}\"?", branch);
        if confirm(&confirm_string) {
            unimplemented!();
        } else {
            final_branch = current;
            println!("{}>> Branch set to \"{}\".{}", G, final_branch, X);
        }
    }
    final_branch
}
