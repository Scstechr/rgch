use crate::{
    ansi::{
        arrows::RET,
        colors::{R, U, X, Y},
        moves::pos_x,
        others::{SAVE, TAB},
        seg::SH,
    },
    arg::opt_set,
};
use std::process::exit;
use termion::terminal_size;

const POS_X_NAME: u64 = 9;
const POS_X_SAVE: u64 = 29;
const POS_X_TYPE: u64 = 32;
const POS_X_HELP: u64 = 39;
const POS_X_SHRT: u64 = 9;

pub fn help() {
    println!(
        "\n{u}Usage:{x}\n\n{t}rgch {y}[FLAGS] [OPTIONS] {r}<INPUT>{x}",
        u = U,
        r = R,
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
                SAVE.to_string()
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
                    format!("<{}>", opt.types.to_uppercase())
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

fn wide_match(category: &str) {
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
            let save_flg = if opt.save {
                format!("{x} {s} ", s = SAVE, x = pos_x(POS_X_SAVE))
            } else {
                "".to_string()
            };
            let types = format!("{x} {t} ", x = pos_x(POS_X_TYPE), t = opt.types);
            println!(
                "{p} {sh} {y}{s}--{l}{reset}{t} {x} {e}{f}",
                sh = SH,
                p = pos_x(POS_X_NAME),
                s = s_string,
                l = opt.long,
                t = types.to_uppercase(),
                x = pos_x(POS_X_HELP),
                e = opt.exp,
                f = save_flg,
                y = Y,
                reset = X,
            );
        }
    }
}

fn wide_help() {
    print!(
        "{pos}{x}   {u}Name            {x}",
        pos = pos_x(POS_X_NAME),
        u = U,
        x = X
    );
    print!(
        "{pos}{x} {u}Save{x}",
        pos = pos_x(POS_X_SAVE - 2),
        u = U,
        x = X
    );
    print!(
        "{pos}{x} {u}Type   {x}",
        pos = pos_x(POS_X_TYPE),
        u = U,
        x = X
    );
    println!(
        "{pos}{x} {u}Explainations                 {x}",
        pos = pos_x(POS_X_HELP),
        u = U,
        x = X
    );
    wide_match("create");
    wide_match("branch");
    wide_match("change");
    wide_match("remote");
    wide_match("extras");
}
