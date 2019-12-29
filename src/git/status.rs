use crate::{
    ansi::{
        colors::{G, R, X},
        others::TAB,
    },
    proc::{execute, execute_out},
};

fn short_status_info() {
    // print!(
    //     "\n{tab}{g}M {x}: Added, not commited.{x}",
    //     tab = TAB,
    //     g = G,
    //     x = X
    // );
    // print!(" |{r} M{x}: Modified, not added.{x}", r = R, x = X);
    // println!(" |{r}??{x}: Untracked.{x}", r = R, x = X);
    print!("\n{tab}[{g}M {x}] Staged.{x}", tab = TAB, g = G, x = X);
    print!(" [{r} M{x}] Unstaged.{x}", r = R, x = X);
    println!(" [{r}??{x}] Untracked.{x}", r = R, x = X);
}

pub fn short_status() {
    execute("git status --short");
    short_status_info();
}

pub fn status() -> bool {
    let output = execute_out("git status -s");
    if !output.is_empty() {
        true
    } else {
        println!("{}Status clean.{}", G, X);
        false
    }
}

pub fn check_status() -> bool {
    let output = execute_out("git status --short");
    if !output.is_empty() {
        true
    } else {
        println!("{}Status clean.{}", G, X);
        false
    }
}
