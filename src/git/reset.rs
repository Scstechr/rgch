use crate::proc::execute_mute;

pub fn reset() {
    execute_mute("git reset");
}
