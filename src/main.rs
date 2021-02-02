use std::str::FromStr;

use rusty_ls8::*;

fn main() {
    let input = "10000010
00000000
00001000
01000111
00000000
00000001";
    
    let mut vm = VM::from_str(input).expect("Failed to parse input");

    vm.run();
}
