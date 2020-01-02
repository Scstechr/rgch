use crate::{
    ansi::{moves::pos_x, others::SQRE},
    proc::{execute_out, parse},
};
use chrono::{Date, Local};
use std::process::exit;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const POS_X: u64 = 15;

pub fn short_version() {
    let (git_rev_parse, _) = execute_out("git rev-parse HEAD");
    let git_rev = parse(git_rev_parse, 0, 8);
    let date: Date<Local> = Local::today();
    println!("RGCH: v{v} ({g} {d})", d = date, v = VERSION, g = git_rev,);
}
pub fn version() {
    let (rustc_version, _) = execute_out("rustc --version");
    let (fmt_version, _) = execute_out("cargo fmt --version");
    let (clippy_version, _) = execute_out("cargo clippy --version");
    print!(
        "  Compiled w/{x}{a} {r}{x}{a} {f}{x}{a} {c}",
        a = SQRE,
        r = rustc_version,
        x = pos_x(POS_X),
        f = fmt_version,
        c = clippy_version,
    );
    exit(0);
}
