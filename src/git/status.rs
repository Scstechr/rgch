use crate::{
    ansi::{
        colors::{FG, FR, G, X},
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
    print!("\n{tab}[{g}A {x}] New.{x}", tab = TAB, g = FG, x = X);
    print!("{tab}[{g}M {x}] Staged.{x}", tab = TAB, g = FG, x = X);
    print!(" [{r} M{x}] Unstaged.{x}", r = FR, x = X);
    println!(" [{r}??{x}] Untracked.{x}", r = FR, x = X);
    print!("{tab}[{r} D{x}] Deleted.{x}", tab = TAB, r = FR, x = X);
    print!("{tab}[{r}UU{x}] Unmerged.{x}", tab = TAB, r = FR, x = X);
    println!(" [{r}DU{x}] Deleted & Unmerged.{x}", r = FR, x = X);
}

pub fn short_status() {
    execute("git status --short");
    short_status_info();
}

pub fn status() {
    execute("git status -s");
}

pub fn check_status() -> bool {
    let (output, _) = execute_out("git status --short");
    if !output.is_empty() {
        true
    } else {
        println!("{}Status clean.{}", G, X);
        false
    }
}
