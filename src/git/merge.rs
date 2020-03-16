use crate::git::{branch, checkout, pull};
use crate::misc::beep;
use crate::proc;
use crate::Opt;
use std::collections::HashMap;

pub fn merge<S: ::std::hash::BuildHasher + Default>(args: &HashMap<String, Opt, S>) {
    beep();
    println!("EXPERIMENTAL FEATURE!");
    let branch = branch::get_branch();
    let mut args_copy: HashMap<String, Opt> = HashMap::new();
    for (a, o) in args.iter() {
        args_copy.insert(
            a.to_string(),
            match a as &str {
                "branch" => Opt {
                    save: o.save,
                    flag: o.flag,
                    value: "master".to_string(),
                },
                _ => Opt {
                    save: o.save,
                    flag: o.flag,
                    value: o.value.clone(),
                },
            },
        );
    }
    // args["branch"].value = "master".to_string();
    // checkout::checkout(&args["branch"].value);
    // pull::pull(&args["remote"].value, &args["branch"].value, false);
    // let command = format!("git merge {} --no-ff", branch);
    // proc::execute(&command);
    // branch::delete_branch(&branch);
}
