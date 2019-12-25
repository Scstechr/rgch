pub mod arg;
pub mod colors;
pub mod proc;

#[derive(Debug)]
pub struct Opt {
    flag: bool,
    value: &'static str,
}

#[derive(Debug)]
pub struct Arg {
    short: &'static str,
    long: &'static str,
    types: &'static str,
    flag: bool,
    value: &'static str,
    exp: &'static str,
}
