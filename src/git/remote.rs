use crate::ansi::{
    colors::{G, R, X},
    others::ARS,
};
use crate::git::{format, git_path_check, set_url};
use crate::misc::{beep, confirm};
use crate::proc::{execute, execute_out};

pub fn get_remote_list() -> Vec<String> {
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

fn add_remote(remote: &str) {
    let url = set_url("");
    let cmd = format!("git remote add {} {}", remote, url);
    execute(&cmd);
}

pub fn set_remote(remote: &str, path: &str) -> String {
    git_path_check(&path);
    let mut final_remote = remote.to_string();
    let remotes = get_remote_list();
    if !remotes.iter().any(|r| r == remote) {
        beep();
        println!(
            "{r}{a}Remote repository {b}{r} not found.{x}",
            r = R,
            a = ARS,
            b = format(&remote),
            x = X
        );
        let confirm_string = format!("Add remote repository {}", remote);
        if confirm(&confirm_string) {
            add_remote(&remote);
        // unimplemented();
        } else {
            final_remote = "origin".to_string();
            println!(
                "{g}{a}Remote repository set to {b}{g}.{x}",
                g = G,
                a = ARS,
                b = format(&remote),
                x = X
            );
        };
    }
    final_remote
}
