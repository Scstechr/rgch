pub mod add;
pub mod branch;
pub mod checkout;
pub mod clone;
pub mod commit;
pub mod diff;
pub mod init;
pub mod log;
pub mod pull;
pub mod push;
pub mod remote;
pub mod reset;
pub mod status;

use std::path::Path;
// use crate::misc::{warning, confirm, exit_msg};
// use crate::git::init::init;

pub fn git_path_check(path: &str) {
    if !Path::new(&".git").exists() {
        let string = format!("Path `{}` does not have a `.git` directory!", path);
        crate::misc::warning(&string);
        let question = "Initialize".to_string();
        if crate::misc::confirm(&question) {
            crate::git::init::init(path);
        } else {
            crate::misc::exit_msg(1);
        }
    }
}
