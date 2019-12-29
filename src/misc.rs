extern crate termion;

use crate::ansi::{
    colors::{R, U, X, Y},
    moves::up_delete,
    others::ARS,
};
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
    let question = question.replace('`', U);
    let string = format!("{}>> {}{}{}? -> press: [y/n] {}", Y, question, X, Y, X);
    println!("{}\x1b[A", string);
    let mut f = true;
    let mut escape = true;
    let exit_a = Event::Key(Key::Ctrl('c'));
    let exit_b = Event::Key(Key::Ctrl('d'));
    let yes_a = Event::Key(Key::Char('y'));
    let yes_b = Event::Key(Key::Char('Y'));
    let no_a = Event::Key(Key::Char('n'));
    let no_b = Event::Key(Key::Char('N'));
    let mut abort = false;
    while escape {
        escape = false;
        let stdin = stdin();
        #[allow(unused_variables)]
        let stdout = stdout().into_raw_mode().unwrap();
        for evt in stdin.events() {
            let press = evt.unwrap();
            if press == exit_a || press == exit_b {
                println!("\n{r}{a}Aborting!{x}", r = R, a = ARS, x = X);
                abort = true;
                break;
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
    if abort {
        let _ = stdout().flush();
        print!("\x1b[G");
        exit(1);
    }
    print!("{}", up_delete(1));
    f
}

pub fn input(question: &str) -> String {
    print!("{}>> {}: {}", Y, question, X);
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
