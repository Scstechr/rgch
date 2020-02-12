#![allow(dead_code)]
#![allow(unused_variables)]
use crate::git::git_path_check;
use crate::proc::execute_out;

fn get_remote_list() -> Vec<String> {
    let (output, _) = execute_out("git remote");
    let mut remotes: Vec<String> = Vec::new();
    let remotes_strs: Vec<&str> = output.split('\n').collect();
    for remote in remotes_strs {
        if !remote.is_empty() {
            remotes.push(remote.replace(" ", "").to_string());
        }
    }
    remotes
}

pub fn set_remote(remote: &str, path: &str) -> String {
    git_path_check(&path);
    println!("{:#?}", get_remote_list());
    "origin".to_string()
}
