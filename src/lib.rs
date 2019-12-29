pub mod ansi;
pub mod arg;
pub mod error;
pub mod git;
pub mod misc;
pub mod proc;

#[derive(Debug)]
pub struct Opt {
    flag: bool,
    value: String,
}

#[derive(Debug)]
pub struct Arg {
    short: &'static str,
    long: &'static str,
    types: &'static str,
    flag: bool,
    value: String,
    exp: &'static str,
}
