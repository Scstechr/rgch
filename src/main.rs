extern crate rgch;

use rgch::{
    arg::{save, set_default, set_git_dir},
    git::{
        add::{add, silence_add},
        branch::set_branch,
        clone::clone,
        commit::{amend, check_raw_commit},
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
    misc::show,
    version::{short_version, version},
};
#[allow(unused_imports)]
use std::process::exit;

fn main() {
    short_version();

    let args = set_default();

    if args["show-args"].flag {
        show(&args);
    }

    // can be operated without .git folder
    match (
        args["version"].flag,
        args["help"].flag,
        args["clone"].flag,
        args["init"].flag,
    ) {
        (true, _, _, _) => version(),
        (_, true, _, _) => help(),
        (_, _, true, _) => clone(
            &args["clone"].value,
            &args["branch"].value,
            args["branch"].flag,
        ),
        (_, _, _, true) => init(&args["gitdir"].value),
        (_, _, _, _) => (),
    }

    set_git_dir(&args["gitdir"].value);

    match (args["log"].flag, args["ls"].flag) {
        (true, _) => log(),
        (_, true) => ls(),
        (_, _) => (),
    }

    if args["save"].flag {
        save(&args);
    }

    if args["amd"].flag {
        amend();
    } else {
        let remote = if args["push"].flag {
            set_remote(&args["remote"].value, &args["gitdir"].value)
        } else {
            "origin".to_string()
        };

        if args["pull"].flag {
            pull(&remote, &args["branch"].value, true);
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
            if args["merge"].value != args["branch"].value
                && !is_status_clean()
                && args["merge"].value == "master"
            {
                check_raw_commit(&args);
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
            args["branch"].value.clone()
        };

        if args["push"].flag {
            push(&remote, &branch);
        }
    }

    println!();
}
