use std::env;

mod loader;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => (),
        _ => panic!("Usage: \"cargo run $BINARY\"")
    }
    loader::elf::init::init(&args[1]);
    println!("Hello, world!");
}
