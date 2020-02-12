use crate::proc::execute;

pub fn push(branch: &str, remote: &str) {
    let string = format!("git push origin {}", branch);
    execute(&string);
}
