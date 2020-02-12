use crate::proc::execute;

pub fn push(remote: &str, branch: &str) {
    let string = format!("git push {} {}", remote, branch);
    execute(&string);
}
