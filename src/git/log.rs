use crate::proc::execute;

const LOG_CMD: &str = "git log --stat --oneline --graph --decorate";

pub fn log() {
    execute(LOG_CMD);
}
