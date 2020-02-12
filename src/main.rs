extern crate rgch;

use rgch::{
    arg::{parse_arguments, save, set_default, set_git_dir},
    git::{
        add::{add, silence_add},
        branch::set_branch,
        clone::clone,
        commit::{amend, commit},
        diff::diff,
        init::init,
        log::log,
        pull::pull,
        push::push,
        remote::set_remote,
        reset::reset,
        status::{check_status, is_status_clean, short_status},
    },
    help::help,
    version::{short_version, version},
};
#[allow(unused_imports)]
use std::process::exit;

fn main() {
    short_version();

    let args = parse_arguments();
    let args = set_default(&args);
    if args["save"].flag {
        save(&args);
    }

    // for arg in &args {
    //     println!("{:?}", arg);
    // }
    // exit(0);
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
        let branch = set_branch(&args["branch"].value, &args["gitdir"].value);
        let remote = set_remote(&args["remote"].value, &args["gitdir"].value);

        if args["log"].flag {
            log();
        }

        if args["pull"].flag {
            pull(&args["remote"].value, &args["branch"].value);
        }

        if is_status_clean() {
            diff(args["verbose"].flag);
        }

        if args["add"].flag {
            add(&args["add"].value, args["force"].flag);
        } else {
            silence_add(&args["add"].value, args["force"].flag);
        }

        if args["commit"].flag {
            commit(&args["commit"].value);
        } else {
            if check_status() {
                short_status();
            }
            reset();
        }
        if args["push"].flag {
            push(&branch, &remote);
        }
    }
}
