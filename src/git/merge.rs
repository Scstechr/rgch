use crate::arg::save;
// use crate::error::unimplemented;
use crate::git::{branch, checkout, commit::commit, pull, status::is_status_clean};
use crate::misc::{confirm, warning};
use crate::proc;
use crate::Opt;
use std::collections::HashMap;
use std::path::Path;
use std::process::exit;

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
        checkout::checkout(&args_c["branch"].value);
        pull::pull(&args_c["remote"].value, &args_c["branch"].value, false);
        let command = format!("git merge {} --no-ff", branch);
        proc::execute(&command);
        let command = format!("Delete branch {}", branch);
        if confirm(&command) {
            branch::delete_branch(&branch);
        }
        if Path::new("./.config.toml").exists() {
            save(&args_c);
        }
    } else {
        let msg = format!("Cannot merge {} into {}.", branch, args["merge"].value);
        warning(&msg);
    }
}

pub fn merge<S: ::std::hash::BuildHasher + Default>(args: &HashMap<String, Opt, S>) {
    let branch = branch::get_branch();
    if branch != "master" {
        if !is_status_clean() {
            commit(&args["commit"].value);
        }
        merge_not_master(args);
    } else {
        println!("merging {:?} to master?", args["merge"].value);
        if !is_status_clean() && branch::branch_exists(&args["merge"].value) {
            let msg = format!(
                "Abort due to: branch {} exists and some changes were made in {}",
                args["merge"].value, args["branch"].value
            );
            warning(&msg);
            exit(0);
        } else {
            branch::make_branch(&args["merge"].value);
        }
    }
}
