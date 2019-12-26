pub mod arg;
pub mod colors;
pub mod error;
pub mod git;
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
