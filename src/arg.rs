use std::env;
use crate::{Config, Arg};

pub fn parse_defaults() -> Config {
    let config = Config {
       commit: true,
       push: true,
    };
    config
}

pub fn parse_arguments() -> Vec<String> {
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
