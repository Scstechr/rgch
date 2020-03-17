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
        ls_files::ls,
        merge::merge,
        pull::pull,
        push::push,
        remote::set_remote,
        reset::reset,
        status::{check_status, is_status_clean, short_status},
    },
    help::help,
    misc::warning,
    version::{short_version, version},
};
#[allow(unused_imports)]
use std::process::exit;

fn main() {
    short_version();

    let args = parse_arguments();
    let args = set_default(&args);

    if args["show-args"].flag {
        match args["show-args"].value.len() {
            0 => println!("{:?}", args),
            _ => println!("{:#?}", args[&args["show-args"].value]),
        };
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
            if !is_status_clean() && args["merge"].value == "master" {
                if args["branch"].value == "master" && !args["no-raw"].flag {
                    commit(&args["commit"].value);
                } else {
                    warning("Raw commit not allowed in master branch");
                }
            }
            merge(&args);
            args["merge"].value.clone()
        } else {
            if args["commit"].flag {
                if args["branch"].value == "master" && !args["no-raw"].flag {
                    commit(&args["commit"].value);
                } else {
                    warning("Raw commit not allowed in master branch");
                }
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
