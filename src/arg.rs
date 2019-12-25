
use crate::Config;

pub fn parse_defaults() -> Config {
    let config = Config {
       commit: true,
       push: true,
    };
    config
}
