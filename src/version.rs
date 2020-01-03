use crate::ansi::others::SQRE;
use std::process::exit;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BLANK: &str = "            ";
const GITREV: &str = "df70e311";
const DATE: &str = "2020-01-03";
const RUSTC: &str = "rustc 1.40.0 (73528e339 2019-12-16)\n";
const FMT: &str = "clippy 0.0.212 (c8e3cfbd 2019-10-28)\n";
const CLIPPY: &str = "rustfmt 1.4.9-stable (33e36670 2019-10-07)\n";

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
