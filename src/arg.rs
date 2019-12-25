use std::env;
use std::collections::HashMap;
use crate::{Config, Arg};

fn opt_set() -> Vec<Arg> {
    let mut opts: Vec<Arg> = Vec::new();
    opts.push(Arg{
        short: "c",
        long: "commit",
        types: "flag",
        flags: false,
        value: "None",
        exp: "Commit",
    });
    opts.push(Arg{
        short: "p",
        long: "push",
        types: "flag",
        flags: false,
        value: "None",
        exp: "Push",
    });
    // println!("{:?}", opts);
    opts
}

pub fn parse_defaults() -> Config {
    let config = Config {
       commit: true,
       push: true,
    };
    config
}

fn search(arg: &str, options: &mut Vec<Arg>) {
    let mut hit = false;
    for mut opt in options.iter_mut() {
        if arg.contains(opt.long) { 
            opt.flags = true;
            println!("{:?}", opt);
            hit = true;
            break;
        }
        else if arg == opt.short { 
            opt.flags = true;
            println!("{:?}", opt);
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
    // println!("{:?}", args);
    for arg in args.iter() {
        if arg.starts_with("--") {
            search(&arg, &mut options);
        } 
        else if arg.starts_with("-") {
            for arg_c in arg.split("").filter(|&c| c != "").filter(|&c| c != "-") {
                search(&arg_c, &mut options);
                // let mut hit = false;
                // for mut opt in options.iter_mut() {
                //     if aa.contains(opt.short) { 
                //         opt.flags = true;
                //         println!("{:?}", opt);
                //         hit = true;
                //         break;
                //     }
                // }
                // if !hit {
                //     panic!("Invalid argument: {}", a);
                // };
            }
        }
    }
    options
}
