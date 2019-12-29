use crate::proc::execute;

pub fn add(f: &str) {
    let command = format!("git add {}", f);
    execute(&command);
}
