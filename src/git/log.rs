use crate::proc::execute;
use std::process::exit;

const LOG_CMD: &str = "git log --stat --oneline --graph --decorate";

pub fn log() {
    execute(LOG_CMD);
    exit(0);
}
