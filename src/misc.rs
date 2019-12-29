extern crate termion;
extern crate termios;

use crate::colors::{G, X};
use std::{
    io::{stdin, stdout, Write},
    process::Command,
};
use termion::event::{Event, Key};
use termion::input::TermRead;

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
    while escape {
        let stdin = stdin();
        for evt in stdin.events() {
            let press = evt.unwrap();
            if press == Event::Key(Key::Ctrl('c')) {
                println!("Aborting");
                escape = false;
                break;
            } else if press == Event::Key(Key::Char('y')) || press == Event::Key(Key::Char('Y')) {
                escape = false;
                break;
            } else if press == Event::Key(Key::Char('n')) || press == Event::Key(Key::Char('N')) {
                f = false;
                escape = false;
                break;
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
