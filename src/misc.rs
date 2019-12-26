extern crate termios;

use std::io;
use std::io::{Read, Write};
use std::process::Command;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

pub fn beep() {
    Command::new("sh")
        .arg("-c")
        .arg("echo \"\\07\"")
        .spawn()
        .expect("Failed to execute");
}

pub fn confirm(question: &str) -> bool {
    let stdin = 0;
    let mut termios = Termios::from_fd(stdin).unwrap();
    termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0; 1];
    let string = format!("{} [y/n]: ", question);
    print!("{}", string);
    let mut flag = true;
    loop {
        stdout.lock().flush().unwrap();
        reader.read_exact(&mut buffer).unwrap();
        match buffer[0] as char {
            'Y' | 'y' => {
                tcsetattr(stdin, TCSANOW, &termios).unwrap();
            }
            'N' | 'n' => {
                flag = false;
                tcsetattr(stdin, TCSANOW, &termios).unwrap();
            }
            _ => {
                continue;
            }
        }
        break;
    }
    flag
}
