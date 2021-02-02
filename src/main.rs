use std::fs;
use std::env;
use std::process;
use std::str::FromStr;

use rusty_ls8::*;

fn main() {
    let mut args = env::args();
    args.next();
    
    let filename = match args.next() {
        Some(f) => f,
        None => {
            eprintln!("No filename found.");
            process::exit(1);
        }
    };

    let input = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Failed to read file contents.");
            process::exit(1);
        }
    };

    let mut vm = match VM::from_str(&input) {
        Ok(vm) => vm,
        Err(_) => {
            eprintln!("Failed to initialize VM.");
            process::exit(1);
        }
    };

    vm.run();
}
