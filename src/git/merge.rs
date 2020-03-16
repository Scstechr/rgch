use crate::git::{branch, checkout, pull};
use crate::misc::beep;
use crate::proc;

pub fn merge(given_remote: &str, given_branch: &str) {
    beep();
    println!("EXPERIMENTAL FEATURE!");
    let branch = branch::get_branch();
    checkout::checkout("master");
    pull::pull(given_remote, given_branch, false);
    let command = format!("git merge {} --no-ff", branch);
    proc::execute(&command);
    branch::delete_branch(&branch);
}
