use std::env;

mod loader;
mod arch;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => (),
        _ => panic!("Usage: \"cargo run $BINARY\"")
    }
    loader::elf::init::init(&args[1]);
    arch::x86::x86_64::cpu::init();
    println!("Hello, world!");
}
