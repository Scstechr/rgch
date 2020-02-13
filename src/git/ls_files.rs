#![allow(clippy::type_complexity)]
#![allow(clippy::map_entry)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::ansi::{
    colors::{G, R, X},
    others::ARS,
};
use crate::ansi::{
    others::{END, TAB, TO},
    seg::SH,
};
use crate::git::{format, git_path_check, set_url};
use crate::misc::{beep, confirm};
use crate::proc::{execute, execute_out};
use std::collections::HashMap;

const LAYER: usize = 5;
const LEVEL: [usize; LAYER] = [0, 1, 2, 3, 4];
static COUNT: [usize; 2] = [0, 0];

fn parse(output: Vec<&str>) -> HashMap<&str, HashMap<&str, HashMap<&str, HashMap<&str, &str>>>> {
    let mut d: HashMap<&str, HashMap<&str, HashMap<&str, HashMap<&str, &str>>>> = HashMap::new();
    for f in &output {
        let s: Vec<&str> = f.split('/').collect();
        if !d.contains_key(&s[0]) {
            let h: HashMap<&str, HashMap<&str, HashMap<&str, &str>>> = HashMap::new();
            d.insert(s[0], h);
        }
        if s.len() > 1 && !d[&s[0]].contains_key(&s[1]) {
            let h: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
            // h.insert(".", ".");
            d.get_mut(&s[0]).unwrap().insert(s[1], h);
        }
        if s.len() > 2 && !d[&s[0]][&s[1]].contains_key(&s[2]) {
            let h: HashMap<&str, &str> = HashMap::new();
            d.get_mut(&s[0])
                .unwrap()
                .get_mut(&s[1])
                .unwrap()
                .insert(s[2], h);
        }
        if s.len() > 3 && !d[&s[0]][&s[1]][&s[2]].contains_key(&s[3]) {
            d.get_mut(&s[0])
                .unwrap()
                .get_mut(&s[1])
                .unwrap()
                .get_mut(&s[2])
                .unwrap()
                .insert(s[3], "");
        }
    }
    d
}

fn print_out<T: std::fmt::Debug>(d: HashMap<&str, T>) {
    for (key, val) in d.iter() {
        print!("{s}", s = TAB);
        println!("{:?}, {:#?}", key, val);
    }
}

pub fn ls() {
    let (output, _) = execute_out("git ls-files");
    let mut files: Vec<&str> = output.split('\n').collect();
    files.pop(); // remove "" at the end of vector
    let d = parse(files);
    print_out(d);
}
