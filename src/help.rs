use crate::{
    ansi::{
        arrows::RET,
        colors::{S, U, X, Y},
        moves::pos_x,
        others::TAB,
        seg::SH,
    },
    arg::opt_set,
};
use std::process::exit;
use termion::terminal_size;

const POS_X_SAVE: u64 = 18;
const POS_X_TYPE: u64 = 21;
const POS_X_HELP: u64 = 28;
// const BLNK: &str = "                                                                 "; // Blank
const POS_X_SHRT: u64 = 9;

#[allow(dead_code)]
fn set_defaults() {}

fn short_match(category: &str) {
    print!(
        "{t}{u}{c}{x}",
        t = TAB,
        u = U,
        c = category.to_uppercase(),
        x = X
    );
    let options = opt_set();
    for opt in options {
        if opt.category == category {
            let s_string = if opt.short != "" {
                format!("-{s}, ", s = opt.short)
            } else {
                "".to_string()
            };
            let save = if opt.save {
                "\u{2398}".to_string()
            } else {
                "".to_string()
            };

            println!(
                "{p} {seg} {y}{s}--{l} {t} {save}{x}\n{p} {seg} {tab}{r} {e} ",
                p = pos_x(POS_X_SHRT),
                y = Y,
                s = s_string,
                l = opt.long,
                t = if opt.types != "flag" {
                    format!("<{}>", opt.types)
                } else {
                    "".to_string()
                },
                seg = SH,
                save = save,
                tab = TAB,
                r = RET,
                e = opt.exp,
                x = X,
            );
        }
    }
}

fn short_help() {
    short_match("create");
    short_match("branch");
    short_match("change");
    short_match("remote");
    short_match("extras");
}

fn wide_help() {
    print!(
        "{pos}{x} {s}{u}Save{x}",
        pos = pos_x(POS_X_SAVE - 2),
        s = Y,
        u = U,
        x = X
    );
    print!(
        "{pos}{x} {s}{u}Type{x}",
        pos = pos_x(POS_X_TYPE),
        s = Y,
        u = U,
        x = X
    );
    println!(
        "{pos}{x} {s}{u}Explainations{x}",
        pos = pos_x(POS_X_HELP),
        s = Y,
        u = U,
        x = X
    );
    let options = opt_set();
    for opt in options {
        let s_string = if opt.short != "" {
            format!("-{s}, ", s = opt.short)
        } else {
            "".to_string()
        };
        let save_flg = if opt.save {
            format!("{x} \u{2714} ", x = pos_x(POS_X_SAVE))
        } else {
            "".to_string()
        };
        let types = format!("{x} {t} ", x = pos_x(POS_X_TYPE), t = opt.types);
        println!(
            " {s}--{l}{t} {x} {e}{f}",
            s = s_string,
            l = opt.long,
            t = types,
            x = pos_x(POS_X_HELP),
            e = opt.exp,
            f = save_flg
        );
    }
}

pub fn help() {
    println!(
        "\n{s}{u}Usage:{x}\n\n{t}rgch {y}[OPTION]{x}",
        s = S,
        u = U,
        x = X,
        t = TAB,
        y = Y
    );
    println!("\n{u}Options:{x}\n", u = U, x = X);
    let (hsize, _) = terminal_size().unwrap();
    if hsize > 71 {
        wide_help()
    } else {
        short_help()
    }
    exit(0);
}
