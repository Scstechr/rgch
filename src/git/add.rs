use crate::proc::execute;

pub fn add(f: &str, force: bool) {
    let command = if force {
        format!("git add -f {}", f)
    } else {
        format!("git add {}", f)
    };
    execute(&command);
}
