pub mod proc;
pub mod arg;

pub struct Config {
    commit: bool,
    push: bool
}

#[derive(Debug)]
pub struct Arg {
    short: &'static str,
    long: &'static str,
    types: &'static str,
    flags: bool,
    value: &'static str,
    exp: &'static str,
}
