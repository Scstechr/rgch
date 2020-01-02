use crate::{
    git::{
        add::add,
        status::{check_status, short_status},
    },
    misc::input,
    proc::execute,
};

pub fn commit(file: &str, msg: &str, force: bool) {
    if check_status() {
        add(&file, force);
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
