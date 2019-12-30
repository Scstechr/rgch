use crate::{
    misc::warning,
    proc::{execute, execute_mute, execute_out},
};
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
    let command = if force {
        format!("git add -f {}", f)
    } else {
        format!("git add {}", f)
    };
    let (_, ecode) = execute_out(&command);
    execute_mute("git reset");

    // sort of error handling
    if ecode > 0 {
        println!("a");
        if ecode == 1 {
            // tried to add files ignored in `.gitignore`
            let string = format!("Path `{}` is currently ignored in `.gitignore`.", f);
            warning(&string);
            exit(1);
        } else if ecode == 128 {
            // tried to add files that does not exist
            let string = format!("Path `{}` not found.", f);
            warning(&string);
            exit(128);
        } else {
            let string = format!("Unknown error when adding {}.", f);
            warning(&string);
            exit(10);
        }
    }

    execute(&command);
}
