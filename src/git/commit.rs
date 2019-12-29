use crate::{
    git::{
        add::add,
        status::{check_status, short_status},
    },
    misc::input,
    proc::execute,
};

pub fn commit(file: &str) {
    if check_status() {
        add(&file);
        short_status();
        let q = "Enter commit message";
        let s = input(&q);
        let command = if s != "" {
            format!("git commit -m \"{}\"", s)
        } else {
            "git commit -a".to_string()
        };
        execute(&command);
    }
}
