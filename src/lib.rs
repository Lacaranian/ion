#![feature(convert)]
pub mod command;

use std::io;
use command::*;

pub fn repl() {
    let mut input = String::new();
    let _unused = io::stdin().read_line(&mut input);
    let out_wrap = run(input.trim().split_whitespace().collect::<Vec<&str>>());
    if out_wrap.is_some() {
        let out = out_wrap.unwrap();
        if out.stdout.is_empty() {
            println!("{}",out.stderr.trim());
        } else {
            println!("{}",out.stdout.trim());
        }
    } else {
        println!("{} is not a valid command", input.trim());
    }
}
