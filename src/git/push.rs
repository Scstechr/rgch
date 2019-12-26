use crate::proc::execute;

pub fn push(branch: &str) {
    let string = format!("git push origin {}", branch);
    execute(&string);
}
