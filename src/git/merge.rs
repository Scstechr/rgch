use crate::arg::save;
// use crate::error::unimplemented;
use crate::git::{branch, checkout, commit::commit, pull, status::is_status_clean};
use crate::misc::warning;
use crate::proc;
use crate::Opt;
use std::collections::HashMap;
use std::path::Path;

pub fn merge_not_master<S: ::std::hash::BuildHasher + Default>(args: &HashMap<String, Opt, S>) {
    let branch = branch::get_branch();
    if branch != args["merge"].value {
        warning(&"Experimental Feature");
        let mut args_c: HashMap<String, Opt> = HashMap::new();
        for (a, o) in args.iter() {
            args_c.insert(
                a.to_string(),
                match a as &str {
                    "branch" => Opt {
                        save: o.save,
                        flag: o.flag,
                        value: args["merge"].value.clone(),
                    },
                    _ => Opt {
                        save: o.save,
                        flag: o.flag,
                        value: o.value.clone(),
                    },
                },
            );
        }
        if args["merge"].value != "master" {
            branch::set_branch(&args["merge"].value, &args["gitdir"].value);
            if !is_status_clean() {
                commit(&args["commit"].value);
            }
        }
        checkout::checkout(&args_c["branch"].value);
        pull::pull(&args_c["remote"].value, &args_c["branch"].value, false);
        let command = format!("git merge {} --no-ff", branch);
        proc::execute(&command);
        branch::delete_branch(&branch);
        if Path::new("./.config.toml").exists() {
            save(&args_c);
        }
    } else {
        let msg = format!("Cannot merge {} into {}.", branch, args["merge"].value);
        warning(&msg);
    }
}

pub fn merge<S: ::std::hash::BuildHasher + Default>(args: &HashMap<String, Opt, S>) {
    if args["branch"].value != "master" {
        merge_not_master(args);
    } else {
        println!("merging {:?} to master?", args["merge"].value);
    }
}
