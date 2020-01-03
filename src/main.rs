extern crate rgch;

use rgch::{
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
    set::set_git_dir,
    version::{short_version, version},
};

fn main() {
    short_version();
    let args = parse_arguments();
    if args["gitdir"].flag {
        set_git_dir(&args["gitdir"].value);
    }

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
