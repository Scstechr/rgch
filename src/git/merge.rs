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
    branch: &str,
) -> HashMap<String, Opt> {
    let mut args_c: HashMap<String, Opt> = HashMap::new();
    for (a, o) in args.iter() {
        args_c.insert(
            a.to_string(),
            match a as &str {
                "branch" => Opt {
                    save: o.save,
                    flag: o.flag,
                    value: branch.to_string(),
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
    checkout::checkout(&"master");
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
    let mbranch = if confirm(&command) {
        branch::delete_branch(&branch);
        "master".to_string()
    } else {
        println!();
        checkout::checkout(&branch);
        branch
    };
    let args_c = return_args_c(&args, &mbranch);
    if Path::new("./.config.toml").exists() {
        save(&args_c);
    }
}

fn branch_already_exists<S: ::std::hash::BuildHasher + Default>(args: &HashMap<String, Opt, S>) {
    let msg = format!(
        "Branch `{}` already exists, and its unimplemented behavior.",
        args["merge"].value
    );
    warning(&msg);
    exit(0);
}

fn use_branch_instead<S: ::std::hash::BuildHasher + Default>(
    args: &HashMap<String, Opt, S>,
    branch: &str,
) {
    let msg = format!(
        "Nothing changed in `{}` and branch `{}` does not exist.",
        branch, args["merge"].value
    );
    warning(&msg);
    warning(&"Use `rgch -b/--branch <branch name>` instead to switch branch.");
    exit(0);
}

fn cannot_merge<S: ::std::hash::BuildHasher + Default>(
    args: &HashMap<String, Opt, S>,
    branch: &str,
) {
    let msg = format!("Cannot merge `{}` to `{}`.", args["merge"].value, branch);
    warning(&msg);
    exit(0);
}

pub fn merge<S: ::std::hash::BuildHasher + Default>(args: &HashMap<String, Opt, S>) {
    warning(&"Experimental Feature!");
    warning(&"Stricted to merging branches to `master` only.");
    let branch = branch::get_branch();
    if branch != args["merge"].value {
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
            branch_already_exists(args);
        } else {
            use_branch_instead(args, &branch);
        }
    } else {
        cannot_merge(args, &branch);
    }
}
