pub mod proc;
pub mod arg;

pub struct Config {
    commit: bool,
    push: bool
}

pub struct Arg {
    short: String,
    long: String,
    types: String,
    exp: String,
}
