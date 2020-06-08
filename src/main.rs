extern crate elf;

use std::path::PathBuf;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => (),
        _ => panic!("Usage: \"cargo run $BINARY\"")
    }
    let path = PathBuf::from(&args[1]);
    let file = elf::File::open_path(&path).expect("Invalid given executable");
    println!("Hello, world!");
}
