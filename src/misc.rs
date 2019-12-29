extern crate termion;

use crate::colors::{G, X};
use std::{
    io::{stdin, stdout, Write},
    process::{exit, Command},
};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn beep() {
    Command::new("sh")
        .arg("-c")
        .arg("echo \"\\07\"")
        .spawn()
        .expect("Failed to execute");
}

pub fn confirm(question: &str) -> bool {
    let string = format!("{}>> {} (press: [y/n]) {}", G, question, X);
    println!("{}", string);
    let mut f = true;
    let mut escape = true;
    let exit_a = Event::Key(Key::Ctrl('c'));
    let exit_b = Event::Key(Key::Ctrl('d'));
    let yes_a = Event::Key(Key::Char('y'));
    let yes_b = Event::Key(Key::Char('Y'));
    let no_a = Event::Key(Key::Char('n'));
    let no_b = Event::Key(Key::Char('N'));
    while escape {
        escape = false;
        let _stdout = stdout().into_raw_mode().unwrap();
        for evt in stdin().events() {
            let press = evt.unwrap();
            // println!("pressed: {:?}", press);
            if press == exit_a || press == exit_b {
                println!("Aborting");
                exit(1);
            } else if press == yes_a || press == yes_b {
                break;
            } else if press == no_a || press == no_b {
                f = false;
                break;
            } else {
                escape = true;
            }
        }
    }
    f
}

pub fn input(question: &str) -> String {
    print!("\n{}>> {}: {}", G, question, X);
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("-a");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}
