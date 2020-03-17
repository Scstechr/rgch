#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::arg::save;
// use crate::error::unimplemented;
use crate::git::{branch, checkout, commit::commit, pull, status::is_status_clean};
use crate::misc::{self, confirm, warning};
use crate::proc;
use crate::Opt;
use std::collections::HashMap;
use std::path::Path;
use std::process::exit;

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
    pull::pull(&args["remote"].value, &args["branch"].value, false);
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
    if confirm(&command) {
        branch::delete_branch(&branch);
    }
    let args_c = return_args_c(&args);
    // misc::show(&args_c);
    // misc::show(&args);
    // println!("{}, {}", args_c["branch"].value, args["branch"].value);
    // pull::pull(&args_c["remote"].value, &args_c["branch"].value, false);
    if Path::new("./.config.toml").exists() {
        save(&args_c);
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
                commit(&args["commit"].value);
                checkout_pull_merge(args, &branch);
            } else {
                branch::make_branch(&args["merge"].value);
                commit(&args["commit"].value);
                checkout_pull_merge(args, &branch);
            }
        } else if branch::branch_exists(&args["merge"].value) {
            checkout::checkout(&args["merge"].value);
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
    // if branch != "master" {
    //     if !is_status_clean() {
    //     }
    //     merge_not_master(args);
    // } else {
    //     println!("merging {:?} to master?", args["merge"].value);
    //     if !is_status_clean() {
    //         } else {
    //             commit(&args["commit"].value);
    //             merge_not_master(args);
    //         }
    // }
}
