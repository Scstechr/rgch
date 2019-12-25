
use std::env;

use crate::Config;

pub fn parse_defaults() -> Config {
    let config = Config {
       commit: true,
       push: true,
    };
    config
}

pub fn parse_arguments() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}
