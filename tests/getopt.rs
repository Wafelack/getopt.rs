#[macro_use]
extern crate getopt_rs;

use std::{env, process::exit};
use getopt_rs::{LongForm, getopt};

fn main() {
    let mut args = env::args().collect();
    
    while let Some(opt) = getopt(&mut args, "hva:", &[opt!('h', "help", LongForm::Both, true), opt!('a', "all"), opt!('v')]) {
        match opt {
            ('?', _) => exit(1),
            ('h', _) => println!("<help message>"),
            ('v', _) => println!("<version>"),
            ('a', v) => println!("a => {}", v.unwrap()),
            _ => break,
        }
    }
}
