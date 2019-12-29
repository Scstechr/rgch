use crate::git::status::status;
use crate::misc::input;
use crate::proc::execute;

pub fn commit() {
    if status() {
        execute("git add .");
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
