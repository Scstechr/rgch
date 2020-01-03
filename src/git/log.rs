use crate::{
    misc::{exit_msg, warning},
    proc::{execute, execute_out},
};
use std::process::exit;

const LOG_CMD: &str = "git log --stat --oneline --graph --decorate";

pub fn log() {
    let (_, exit_code) = execute_out(LOG_CMD);
    if exit_code > 0 {
        warning("Nothing is commited");
        exit_msg(exit_code);
    } else {
        execute(LOG_CMD);
        exit(0);
    }
}
