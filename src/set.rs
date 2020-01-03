extern crate serde;
extern crate toml;
use crate::{error::invalid_path, Opt};
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn set_git_dir(path: &str) -> bool {
    let path = Path::new(path);
    if env::set_current_dir(&path).is_ok() {
        true
    } else {
        let path = path.to_str().unwrap();
        invalid_path(path);
        false
    }
}

pub fn set_default<S: ::std::hash::BuildHasher + Default>(
    args: &HashMap<String, Opt, S>,
) -> HashMap<String, Opt> {
    // pub fn set_default(args: &HashMap<String, Opt>) -> HashMap<String, Opt> {
    let f = fs::read_to_string(".config.toml");
    let config = match f {
        Ok(file) => file,
        Err(_e) => "".to_string(),
    };

    let mut set_args: HashMap<String, Opt> = HashMap::new();
    // let options = opt_set();
    for (key, val) in args.iter() {
        if !config.is_empty() {
            let default: Result<HashMap<String, Opt>, toml::de::Error> = toml::from_str(&config);
            let default = match default {
                Ok(p) => p,
                Err(e) => panic!("Filed to parse TOML: {}", e),
            };
            set_args.insert(
                key.to_string(),
                Opt {
                    save: val.save,
                    flag: {
                        if !val.flag && val.save {
                            default[key].flag
                        } else {
                            val.flag
                        }
                    },
                    value: { val.value.to_string() },
                },
            );
        } else {
            set_args.insert(
                key.to_string(),
                Opt {
                    save: val.save,
                    flag: val.flag,
                    value: val.value.to_string(),
                },
            );
        }
    }
    set_args
}

#[allow(dead_code)]
pub fn save<S: ::std::hash::BuildHasher + Default>(args: &HashMap<String, Opt, S>) {
    let mut file =
        File::create(".config.toml").unwrap_or_else(|_| panic!("Failed to create .config.toml"));
    let toml = toml::to_string(args).unwrap();
    write!(file, "{}", toml).unwrap_or_else(|_| panic!("failed to write .config.toml"));
}
