use crate::ansi::others::SQRE;
use std::process::exit;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BLANK: &str = "            ";
const GITREV: &str = "d9902301";
const DATE: &str = "2020-04-26";
const RUSTC: &str = "rustc 1.42.0 (b8cedc004 2020-03-09)\n";
const FMT: &str = "clippy 0.0.212 (4ee12063 2020-02-01)\n";
const CLIPPY: &str = "rustfmt 1.4.11-stable (9eb4b564 2020-01-29)\n";

pub fn short_version() {
    println!("RGCH: v{v} ({g} {d})", d = DATE, v = VERSION, g = GITREV);
}
pub fn version() {
    print!(
        "Compiled w/ {a} {r}{b}{a} {f}{b}{a} {c}",
        b = BLANK,
        a = SQRE,
        r = RUSTC,
        f = FMT,
        c = CLIPPY
    );
    exit(0);
}
