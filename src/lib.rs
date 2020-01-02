pub mod ansi;
pub mod arg;
pub mod error;
pub mod git;
pub mod help;
pub mod misc;
pub mod proc;
pub mod version;

use crate::{
    arg::parse_arguments,
    git::{
        add::{add, silence_add},
        branch::set_branch,
        clone::clone,
        commit::commit,
        diff::diff,
        log::log,
        pull::pull,
        push::push,
        reset::reset,
        status::{check_status, is_status_clean, short_status},
    },
    help::help,
    version::{short_version, version},
};

#[derive(Debug)]
pub struct Opt {
    save: bool,
    flag: bool,
    value: String,
}

#[derive(Debug)]
pub struct Arg {
    short: &'static str,
    long: &'static str,
    types: &'static str,
    save: bool,
    flag: bool,
    category: &'static str,
    value: String,
    exp: &'static str,
}

pub fn run() {
    short_version();
    let args = parse_arguments();

    //     for arg in &args {
    //         println!("{:?}", arg);
    //     }

    if args["help"].flag {
        help();
    } else if args["version"].flag {
        version();
    } else if args["clone"].flag {
        clone(
            &args["clone"].value,
            &args["branch"].value,
            args["branch"].flag,
        );
    } else {
        let branch = set_branch(&args["branch"].value, &args["gitdir"].value);

        if args["log"].flag {
            log();
        }

        if args["pull"].flag {
            pull(&args["remote"].value, &args["branch"].value);
        }

        if is_status_clean() {
            diff(args["verbose"].flag);
        }

        if args["commit"].flag {
            commit(
                &args["add"].value,
                &args["commit"].value,
                args["force"].flag,
            );
        } else {
            if args["add"].flag {
                add(&args["add"].value, args["force"].flag);
            } else {
                silence_add(&args["add"].value, args["force"].flag);
            }
            if check_status() {
                short_status();
            }
            reset();
        }
        if args["push"].flag {
            push(&branch);
        }
    }
}
