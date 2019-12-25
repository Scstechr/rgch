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
        exp: "Commit",
    });
    opts.push(Arg {
        short: "p",
        long: "push",
        types: "flag",
        flag: false,
        value: "None",
        exp: "Push",
    });
    // println!("{:?}", opts);
    opts
}

fn search(arg: &str, options: &mut Vec<Arg>) {
    let mut hit = false;
    for mut opt in options.iter_mut() {
        if arg.contains(opt.long) {
            opt.flag = true;
            //         println!("{:?}", opt);
            hit = true;
            break;
        } else if arg == opt.short {
            opt.flag = true;
            //         println!("{:?}", opt);
            hit = true;
            break;
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
    while index < args.len(){
        let arg = &args[index];

    // for arg in args.iter() {
        if arg.starts_with("--") {
            search(&arg, &mut options);
        } else if arg.starts_with("-") {
            for arg_c in arg.split("").filter(|&c| c != "").filter(|&c| c != "-") {
                search(&arg_c, &mut options);
            }
        }
        index += 1;
    }
    options
}
