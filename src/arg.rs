use crate::Arg;
use std::env;

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
        exp: "Print help message.",
    });
    // println!("{:?}", opts);
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

pub fn parse_arguments() -> Vec<Arg> {
    let mut options = opt_set();
    let args: Vec<String> = env::args().collect();
    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
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
    options
}
