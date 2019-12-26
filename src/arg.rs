use crate::colors::{S, U, X};
use crate::{Arg, Opt};
use std::{collections::HashMap, env, process::exit};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn help() {
    println!("{}rgch v{}: Rust implementation of gch{}", S, VERSION, X);
    println!("{}Usage: rgch [OPTION]{}", S, X);
    println!("\n{}{}Options:{}", S, U, X);
    let options = opt_set();
    for opt in options {
        let string = format!("  -{}, --{}\x1b[20G | {}", opt.short, opt.long, opt.exp);
        println!("{}", string);
    }
    exit(0);
}

fn opt_set() -> Vec<Arg> {
    let mut opts: Vec<Arg> = Vec::new();
    opts.push(Arg {
        short: "c",
        long: "commit",
        types: "flag",
        flag: false,
        value: "None",
        exp: "Commit.",
    });
    opts.push(Arg {
        short: "p",
        long: "push",
        types: "flag",
        flag: false,
        value: "None",
        exp: "Push.",
    });
    opts.push(Arg {
        short: "h",
        long: "help",
        types: "flag",
        flag: false,
        value: "None",
        exp: "Show this message and exit.",
    });
    // println!("{:?}", opts);
    opts
}

fn search(arg: &str, options: &mut Vec<Arg>) {
    let mut hit = false;
    for mut opt in options.iter_mut() {
        if arg.contains(opt.long) || arg == opt.short {
            hit = true;
            opt.flag = true;
            break;
        } else {
            hit = false;
        }
    }
    if !hit {
        panic!("Invalid argument: {}", arg);
    };
}

pub fn parse_arguments() -> HashMap<String, Opt> {
    let env_args: Vec<String> = env::args().collect();
    let mut options = opt_set();

    let mut index = 0;
    while index < env_args.len() {
        let arg = &env_args[index];
        if arg.starts_with("--") {
            search(&arg, &mut options);
        } else if arg.starts_with('-') {
            let c_args: Vec<&str> = arg
                .split("")
                .filter(|&c| c != "")
                .filter(|&c| c != "-")
                .collect();
            let mut c_index = 0;
            while c_index < c_args.len() {
                search(&c_args[c_index], &mut options);
                c_index += 1;
            }
        }
        index += 1;
    }

    let mut args: HashMap<String, Opt> = HashMap::new();
    for opt in options {
        args.insert(
            String::from(opt.long),
            Opt {
                flag: opt.flag,
                value: opt.value,
            },
        );
    }
    args
}
