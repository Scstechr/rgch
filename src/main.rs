#![allow(clippy::collapsible_if)]
extern crate rgch;

use rgch::{
    arg::{parse_arguments, save, set_default, set_git_dir},
    git::{
        add::{add, silence_add},
        branch::set_branch,
        clone::clone,
        commit::{amend, check_raw_commit, commit},
        diff::diff,
        init::init,
        log::log,
        ls_files::ls,
        merge::merge,
        pull::pull,
        push::push,
        remote::set_remote,
        reset::reset,
        status::{check_status, is_status_clean, short_status},
    },
    help::help,
    misc::{show, warning},
    version::{short_version, version},
};
#[allow(unused_imports)]
use std::process::exit;

fn main() {
    short_version();

    let args = parse_arguments();
    let args = set_default(&args);

    if args["show-args"].flag {
        show(&args);
    }

    if args["save"].flag {
        save(&args);
    }

    if args["gitdir"].flag {
        set_git_dir(&args["gitdir"].value);
    }

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
    } else if args["init"].flag {
        init(&args["gitdir"].value);
    } else if args["amd"].flag {
        amend();
    } else {
        let remote = if args["push"].flag {
            set_remote(&args["remote"].value, &args["gitdir"].value)
        } else {
            "origin".to_string()
        };

        if args["log"].flag {
            log();
        }

        if args["ls"].flag {
            ls();
        }

        if args["pull"].flag {
            pull(&args["remote"].value, &args["branch"].value, true);
        }

        if !is_status_clean() {
            diff(args["verbose"].flag);
        }

        if args["add"].flag {
            add(&args["add"].value, args["force"].flag);
        } else {
            silence_add(&args["add"].value, args["force"].flag);
        }

        if !args["merge"].flag {
            set_branch(&args["branch"].value, &args["gitdir"].value);
        }

        let branch = if args["merge"].flag {
            if args["merge"].value != args["branch"].value {
                if !is_status_clean() && args["merge"].value == "master" {
                    check_raw_commit(&args);
                }
            }
            merge(&args);
            args["merge"].value.clone()
        } else {
            if args["commit"].flag {
                check_raw_commit(&args);
            } else {
                if check_status() {
                    short_status();
                }
                reset();
            }
            set_branch(&args["branch"].value, &args["gitdir"].value)
        };

        if args["push"].flag {
            push(&remote, &branch);
        }
    }

    println!();
}
