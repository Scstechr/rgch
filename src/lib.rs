pub mod ansi;
pub mod arg;
pub mod error;
pub mod git;
pub mod misc;
pub mod proc;

const VERSION: &str = env!("CARGO_PKG_VERSION");

use crate::{
    ansi::colors::{S, X},
    arg::{help, parse_arguments},
    git::{
        add::add, branch::set_branch, clone::clone, commit::commit, diff::diff, log::log,
        pull::pull, push::push, reset::reset, status::short_status,
    },
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
    println!("{}rgch v{}: Rust implementation of gch{}", S, VERSION, X);
    //     let _config = arg::parse_defaults();
    let args = parse_arguments();

    // for arg in &args {
    //     println!("{:?}", arg);
    // }

    if args["help"].flag {
        help();
    }

    if args["clone"].flag {
        clone(
            &args["clone"].value,
            &args["branch"].value,
            args["branch"].flag,
        );
    }

    let branch = set_branch(&args["branch"].value, &args["gitdir"].value);

    if args["log"].flag {
        log();
    }

    if args["pull"].flag {
        pull(&args["remote"].value, &args["branch"].value);
    }

    diff(args["verbose"].flag);

    if args["commit"].flag {
        commit(&args["add"].value, args["force"].flag);
    } else {
        add(&args["add"].value, args["force"].flag);
        short_status();
        reset();
    }
    if args["push"].flag {
        push(&branch);
    }
}
