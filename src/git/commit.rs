use crate::{
    git::status::{check_status, is_status_clean, short_status},
    misc::input,
    proc::execute,
};

pub fn amend() {
    execute(&"git commit --amend");
}

pub fn check_raw(mbranch: &str) -> bool {
    !is_status_clean() && mbranch == "master"
}

pub fn commit(msg: &str) {
    if check_status() {
        short_status();
        let command = if !msg.is_empty() {
            format!("git commit -m \"{}\"", msg)
        } else {
            let q = "Enter commit message";
            let s = input(&q);
            if s != "" {
                format!("git commit -m \"{}\"", s)
            } else {
                "git commit -a".to_string()
            }
        };
        execute(&command);
    }
}
