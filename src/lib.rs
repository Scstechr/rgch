pub mod ansi;
pub mod arg;
pub mod error;
pub mod git;
pub mod help;
pub mod misc;
pub mod proc;
pub mod set;
pub mod version;

#[derive(Debug)]
pub struct Opt {
    pub save: bool,
    pub flag: bool,
    pub value: String,
}

#[derive(Debug)]
pub struct Arg {
    pub short: &'static str,
    pub long: &'static str,
    pub types: &'static str,
    pub save: bool,
    pub flag: bool,
    pub category: &'static str,
    pub value: String,
    pub exp: &'static str,
}
