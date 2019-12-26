use crate::colors::{R, X};
use crate::misc::beep;
use std::process::exit;

pub fn invalid_argument(arg: &str) {
    beep();
    println!("{}>> Invalid argument: {}{}", R, arg, X);
    exit(1);
}
