#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::ansi::{
    colors::{G, R, X},
    others::ARS,
};
use crate::git::{format, git_path_check, set_url};
use crate::misc::{beep, confirm};
use crate::proc::{execute, execute_out};

pub fn ls() {}
