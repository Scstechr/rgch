use crate::proc::execute;

pub fn checkout(branch: &str) {
    let cmd = format!("git checkout {}", branch);
    execute(&cmd);
}

pub fn checkout_new_branch(branch: &str) {
    let cmd = format!("git checkout -b {}", branch);
    execute(&cmd);
}
