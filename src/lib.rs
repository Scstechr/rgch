pub mod arg;
pub mod colors;
pub mod proc;

pub struct Opt {
    flags: bool,
    value: &'static str,
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
