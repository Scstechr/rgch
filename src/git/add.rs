use crate::proc::{execute, execute_mute};

pub fn silence_add(f: &str, force: bool) {
    let command = if force {
        format!("git add -f {}", f)
    } else {
        format!("git add {}", f)
    };
    execute_mute(&command);
}

pub fn add(f: &str, force: bool) {
    let command = if force {
        format!("git add -f {}", f)
    } else {
        format!("git add {}", f)
    };
    execute(&command);
}
