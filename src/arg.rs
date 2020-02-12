extern crate serde;
extern crate toml;

use crate::{
    error::{invalid_argument, invalid_path},
    git::branch::get_branch,
    Arg, Opt,
};
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
                    value: {
                        if !val.flag && val.save {
                            default[key].value.to_string()
                        } else {
                            val.value.to_string()
                        }
                    },
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

pub fn save<S: ::std::hash::BuildHasher + Default>(args: &HashMap<String, Opt, S>) {
    let mut file =
        File::create(".config.toml").unwrap_or_else(|_| panic!("Failed to create .config.toml"));
    let toml = toml::to_string(args).unwrap();
    write!(file, "{}", toml).unwrap_or_else(|_| panic!("failed to write .config.toml"));
}
pub fn opt_set() -> Vec<Arg> {
    let mut opts: Vec<Arg> = Vec::new();

    // Create
    opts.push(Arg {
        short: "",
        long: "clone",
        types: "string",
        save: false,
        flag: false,
        category: "create",
        value: "".to_string(),
        exp: "Clone remote repository.",
    });
    opts.push(Arg {
        short: "i",
        long: "init",
        types: "flag",
        save: false,
        flag: false,
        category: "create",
        value: "".to_string(),
        exp: "Initialize repository.",
    });

    // Branches
    opts.push(Arg {
        short: "b",
        long: "branch",
        types: "string",
        save: true,
        flag: false,
        category: "branch",
        value: "master".to_string(),
        exp: "Specify branch name.",
    });

    // Changes
    opts.push(Arg {
        short: "l",
        long: "log",
        types: "flag",
        save: false,
        flag: false,
        category: "change",
        value: "".to_string(),
        exp: "Display log.",
    });
    opts.push(Arg {
        short: "a",
        long: "add",
        types: "path",
        save: true,
        flag: false,
        category: "change",
        value: ".".to_string(),
        exp: "Specify path to add.",
    });
    opts.push(Arg {
        short: "c",
        long: "commit",
        types: "flag",
        save: false,
        flag: false,
        category: "change",
        value: "".to_string(),
        exp: "Commit.",
    });

    // Remote
    opts.push(Arg {
        short: "r",
        long: "remote",
        types: "string",
        save: false,
        flag: false,
        category: "remote",
        value: "origin".to_string(),
        exp: "Specify remote repository.",
    });
    opts.push(Arg {
        short: "",
        long: "pull",
        types: "flag",
        save: false,
        flag: false,
        category: "remote",
        value: "".to_string(),
        exp: "Pull (fetch and rebase).",
    });
    opts.push(Arg {
        short: "p",
        long: "push",
        types: "flag",
        save: false,
        flag: false,
        category: "remote",
        value: "".to_string(),
        exp: "Push.",
    });

    // extras
    opts.push(Arg {
        short: "",
        long: "amd",
        types: "flag",
        save: false,
        flag: false,
        category: "extras",
        value: "".to_string(),
        exp: "Change the last commit message.",
    });
    opts.push(Arg {
        short: "g",
        long: "gitdir",
        types: "path",
        save: true,
        flag: false,
        category: "extras",
        value: ".".to_string(),
        exp: "Specify path of `.git`.",
    });
    opts.push(Arg {
        short: "f",
        long: "force",
        types: "flag",
        save: false,
        flag: false,
        category: "extras",
        value: "".to_string(),
        exp: "`-f/--force` option to `add`.",
    });
    opts.push(Arg {
        short: "v",
        long: "verbose",
        types: "flag",
        save: true,
        flag: false,
        category: "extras",
        value: "".to_string(),
        exp: "Verbose option.",
    });
    opts.push(Arg {
        short: "s",
        long: "save",
        types: "flag",
        save: false,
        flag: false,
        category: "extras",
        value: "".to_string(),
        exp: "Save settings to TOML file.",
    });
    opts.push(Arg {
        short: "",
        long: "version",
        types: "flag",
        save: false,
        flag: false,
        category: "extras",
        value: "".to_string(),
        exp: "Version and compiler info.",
    });
    opts.push(Arg {
        short: "h",
        long: "help",
        types: "flag",
        save: false,
        flag: false,
        category: "extras",
        value: "".to_string(),
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
            opt.value = string;
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
        let value = match opt.long {
            "branch" => match get_branch().len() {
                0 => "master".to_string(),
                1 => match opt.value.as_ref() {
                    "@" => get_branch(),
                    _ => opt.value,
                },
                _ => opt.value,
            },
            _ => opt.value,
        };
        args.insert(
            String::from(opt.long),
            Opt {
                save: opt.save,
                flag: opt.flag,
                value,
            },
        );
    }
    println!("\"{}\"", args["branch"].value);
    // args["branch"].value = match get_branch().len() {
    //     0 => "master".to_string(),
    //     _ => get_branch(),
    // };
    // // if args["branch"].value == "@" {
    // //     println!("\"{}\"", get_branch());
    //     // args["branch"] = get_branch();
    // }
    args
}
