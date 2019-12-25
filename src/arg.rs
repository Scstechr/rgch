use std::env;
use std::collections::HashMap;
use crate::{Config, Arg};

fn opt_set() -> Vec<Arg> {
    let mut opts: Vec<Arg> = Vec::new();
    opts
}

pub fn parse_defaults() -> Config {
    let config = Config {
       commit: true,
       push: true,
    };
    config
}

pub fn parse_arguments() -> Vec<String> {
    let options = opt_set();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    for a in args.iter() {
        if a.starts_with("--") {
            println!("case a {}", a);
        } else if a.starts_with("-") {
            println!("case b {}", a);
        }
    }
    args
}
