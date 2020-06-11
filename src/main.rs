use std::env;

mod loader;
mod arch;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => (),
        _ => panic!("Usage: \"cargo run $BINARY\"")
    }
    let elfimg = loader::elf::init::ElfImg::new(&args[1]);
    // arch::x86::x86_64::cpu::init(0, 0);
    println!("Hello, world!");
}
