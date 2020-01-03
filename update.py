import subprocess as sp
from datetime import datetime

date = datetime.today().strftime("%Y-%m-%d")
rev = sp.getoutput("git rev-parse HEAD")[:8]
rst = sp.getoutput("rustc --version")
cli = sp.getoutput("cargo clippy --version")
fmt = sp.getoutput("cargo fmt --version")
rustc = sp.getoutput("rustc --version")

string = """
use crate::ansi::others::SQRE;
use std::process::exit;
"""
print(string)
print(f"const VERSION: &str = env!(\"CARGO_PKG_VERSION\");")
print(f"const BLANK: &str = \"            \";")
print(f"const GITREV: &str = \"{rev}\";")
print(f"const DATE: &str = \"{date}\";")
print(f"const RUSTC: &str = \"{rst}\\n\";")
print(f"const FMT: &str = \"{cli}\\n\";")
print(f"const CLIPPY: &str = \"{fmt}\\n\";")
print()
string = """
pub fn short_version() {
    println!(\"RGCH: v{v} ({g} {d})\", d = DATE, v = VERSION, g = GITREV);
}
pub fn version() {
    print!(
        \"Compiled w/ {a} {r}{b}{a} {f}{b}{a} {c}\",
        b = BLANK,
        a = SQRE,
        r = RUSTC,
        f = FMT,
        c = CLIPPY
    );
    exit(0);
}
"""
print(string)
