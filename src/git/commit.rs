use crate::colors::{G, X};
use crate::git::status::status;
use crate::proc::execute;
use std::io::{stdin, stdout, Write};

pub fn commit() {
    if status() {
        print!("\n{}>> Enter commit message: {}", G, X);
        let mut s = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("-a");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        let command = if s != "" {
            format!("git commit -a -m \"{}\"", s)
        } else {
            "git commit -a".to_string()
        };
        execute(&command);
    }
}
