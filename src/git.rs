pub mod add;
pub mod branch;
pub mod checkout;
pub mod clone;
pub mod commit;
pub mod diff;
pub mod init;
pub mod log;
pub mod ls_files;
pub mod merge;
pub mod pull;
pub mod push;
pub mod remote;
pub mod reset;
pub mod status;

pub const POS_X_ARG: u64 = 31;
pub const MOVE_DEL: &str = "\x1b[1F\x1b[K";

use crate::ansi::{
    arrows::RET,
    colors::{U, X},
    moves::pos_x,
    others::TAB,
};
use std::path::Path;

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

pub fn format(w: &str) -> String {
    format!("{u}{b}{x}", u = U, b = w, x = X)
}

pub fn set_url(given: &str) -> String {
    let url = if given != "" {
        given.to_string()
    } else {
        crate::misc::input("Enter URL (@ for GitHub)")
    };
    let url = url.replace("@", "https://github.com/");
    print!("{x}", x = MOVE_DEL);
    println!(
        "{t}{r} Remote repository set to {a}: {x}{u}{v}{x}",
        a = pos_x(POS_X_ARG),
        t = TAB,
        r = RET,
        u = U,
        x = X,
        v = url
    );
    url
}

pub fn erase_all(arg_path: &str) {
    let path = if arg_path == "" {
        crate::misc::input("Enter path of a file/directory")
    } else {
        arg_path.to_string()
    };
    let path_end = path.clone().pop();
    println!("{:#?}", path_end);

    //     let end = path_copy.pop();
    //     if end == "/" {
    //         println!("Directory");
    //     } else {
    //         println!("File");
    //     }

    // if crate::misc::confirm(&question) {
    //     let command = format!("git filter-branch --tree-filter \"rm -f {}\" HEAD", path);
    //     println!("{}", command);
    // } else {
    //     let path = crate::misc::input("Enter path of a directory");
    //     let command = format!("git filter-branch --tree-filter \"rm -f -r {}\" HEAD", path);
    //     println!("{}", command);
    // }
}
