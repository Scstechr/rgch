use crate::proc::{execute, execute_mute, execute_out};
#[allow(unused_imports)]
use std::process::exit;

pub fn silence_add(f: &str, force: bool) {
    let command = if force {
        format!("git add -f {}", f)
    } else {
        format!("git add {}", f)
    };
    execute_mute(&command);
}

pub fn add(f: &str, force: bool) {
    let command = format!("git add {}", f);
    execute(&command);
    let (output, flag) = execute_out(&command);
    println!("{}, {}", output, output.len());
    exit(0);
    let command = if force {
        format!("git add -f {}", f)
    } else {
        format!("git add {}", f)
    };
    execute(&command);
}
