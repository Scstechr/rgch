// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(dead_code)]

use crate::{
    arg::save,
    git::{
        branch, checkout, commit::commit, pull, remote::get_remote_list, status::is_status_clean,
    },
    misc::{confirm, warning},
    proc, Opt,
};
use std::{collections::HashMap, path::Path, process::exit};

fn return_args_c<S: ::std::hash::BuildHasher + Default>(
    args: &HashMap<String, Opt, S>,
) -> HashMap<String, Opt> {
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
    args_c
}

pub fn checkout_pull_merge<S: ::std::hash::BuildHasher + Default>(
    args: &HashMap<String, Opt, S>,
    branch: &str,
) {
    checkout::checkout(&args["branch"].value);
    if !get_remote_list().is_empty() {
        pull::pull(&args["remote"].value, &args["branch"].value, false);
    }
    let command = if branch != "master" {
        format!("git merge {} --no-ff", branch)
    } else {
        format!("git merge {} --no-ff", args["merge"].value)
    };
    proc::execute(&command);
    let branch = if branch == "master" {
        args["merge"].value.clone()
    } else {
        branch.to_string()
    };
    let command = format!("Delete branch `{}`", branch);
    let flag = confirm(&command);
    if flag {
        branch::delete_branch(&branch);
    }
    let args_c = return_args_c(&args);
    if Path::new("./.config.toml").exists() {
        save(&args_c);
    }
    if !flag {
        println!();
        checkout::checkout(&branch);
    }
}

pub fn merge<S: ::std::hash::BuildHasher + Default>(args: &HashMap<String, Opt, S>) {
    warning(&"Experimental Feature");
    let branch = branch::get_branch();
    if branch != args["merge"].value {
        // println!(" current    : {:?}", branch);
        // println!(" args-branch: {:?}", args["branch"]);
        // println!(" args-merge : {:?}", args["merge"]);
        if !is_status_clean() {
            if branch::branch_exists(&args["merge"].value) {
                if args["merge"].value != "master" {
                    let msg = format!(
                        "Branch `{}` exists and some changes were made in `{}`.",
                        args["merge"].value, branch
                    );
                    warning(&msg);
                    exit(0);
                }
            } else {
                branch::make_branch(&args["merge"].value);
            }
            commit(&args["commit"].value);
            checkout_pull_merge(args, &branch);
        } else if branch::branch_exists(&args["merge"].value) {
            checkout::checkout(&args["merge"].value);
            checkout_pull_merge(args, &args["merge"].value);
        } else {
            let msg = format!(
                "Nothing changed in `{}` and branch `{}` does not exist.",
                branch, args["merge"].value
            );
            warning(&msg);
            exit(0);
        }
    } else {
        let msg = format!("Cannot merge `{}` to `{}`.", args["merge"].value, branch);
        warning(&msg);
        exit(0);
    }
}
