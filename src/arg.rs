use crate::{
    ansi::colors::{S, U, X},
    error::invalid_argument,
    Arg, Opt,
};
use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
    process::exit,
};

fn set_defaults() {}

pub fn help() {
    println!("{}Usage: rgch [OPTION]{}", S, X);
    println!("\n{}{}Options:{}", S, U, X);
    let options = opt_set();
    for opt in options {
        let string = if opt.short != "" {
            format!("  -{}, --{}\x1b[20G | {}", opt.short, opt.long, opt.exp)
        } else {
            format!("  --{}\x1b[20G | {}", opt.long, opt.exp)
        };
        println!("{}", string);
    }
    exit(0);
}

fn opt_set() -> Vec<Arg> {
    let mut opts: Vec<Arg> = Vec::new();
    opts.push(Arg {
        short: "b",
        long: "branch",
        types: "string",
        flag: false,
        value: "master".to_string(),
        exp: "Specify branch name.",
    });
    opts.push(Arg {
        short: "c",
        long: "commit",
        types: "flag",
        flag: false,
        value: "None".to_string(),
        exp: "Commit.",
    });
    opts.push(Arg {
        short: "p",
        long: "push",
        types: "flag",
        flag: false,
        value: "None".to_string(),
        exp: "Push.",
    });
    opts.push(Arg {
        short: "v",
        long: "verbose",
        types: "flag",
        flag: false,
        value: "None".to_string(),
        exp: "Verbose option.",
    });
    opts.push(Arg {
        short: "g",
        long: "gitdir",
        types: "path",
        flag: false,
        value: "None".to_string(),
        exp: "Specify path of `.git`.",
    });
    opts.push(Arg {
        short: "",
        long: "clone",
        types: "string",
        flag: false,
        value: "None".to_string(),
        exp: "Clone remote repository.",
    });

    opts.push(Arg {
        short: "h",
        long: "help",
        types: "flag",
        flag: false,
        value: "None".to_string(),
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
        invalid_argument(arg);
    };
}

fn search_and_set(arg: &str, string: String, options: &mut Vec<Arg>) {
    let mut hit = false;
    for mut opt in options.iter_mut() {
        if arg.contains(opt.long) || arg == opt.short {
            hit = true;
            opt.flag = true;
            opt.value = string.to_string();
            break;
        } else {
            hit = false;
        }
    }
    if !hit {
        panic!("Invalid argument: {}", arg);
    };
}

fn seperate_chars(arg: &str) -> Vec<&str> {
    let c_args: Vec<&str> = arg
        .split("")
        .filter(|&c| c != "")
        .filter(|&c| c != "-")
        .collect();
    c_args
}

pub fn parse_arguments() -> HashMap<String, Opt> {
    let env_args: Vec<String> = env::args().collect();
    let mut options = opt_set();

    let mut index = 1;
    while index < env_args.len() {
        let arg = &env_args[index];
        if arg.starts_with("--") {
            search(&arg, &mut options);
        } else if arg.starts_with('-') {
            let c_args = seperate_chars(arg);
            let mut c_index = 0;
            while c_index < c_args.len() {
                search(&c_args[c_index], &mut options);
                c_index += 1;
            }
        } else if index > 0 {
            let last_arg = &env_args[index - 1];
            if last_arg.starts_with("--") {
                search_and_set(last_arg, arg.to_string(), &mut options);
            } else if last_arg.starts_with('-') {
                let last_c_arg = seperate_chars(last_arg);
                let last_c_arg = last_c_arg.last().cloned().unwrap();
                search_and_set(last_c_arg, arg.to_string(), &mut options);
            } else {
                invalid_argument(arg);
            }
        } else {
            invalid_argument(arg);
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
