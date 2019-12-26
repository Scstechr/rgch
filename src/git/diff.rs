use crate::git::{reset::reset, status::status};
use crate::proc::{execute, execute_mute};

const DIFFCMD: &str = "git diff --cached --ignore-all-space --ignore-blank-lines";

pub fn diff(verbose: bool) {
    if status() {
        if !verbose {
            execute("git diff --stat");
        } else {
            execute_mute("git add .");
            execute(DIFFCMD);
        }
        reset();
    }
}
