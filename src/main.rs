use std::env;

mod loader;
mod arch;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => (),
        _ => panic!("Usage: \"cargo run $BINARY\"")
    }
    let mut elfimg = loader::elf::load::ElfImg::new(&args[1]);
    match elfimg.load() {
        Err(_e) => panic!("Fata error while loading image"),
        Ok(_o) => ()
    }
    // let proc = arch::x86::x86_64::cpu::init(0, 0);
    println!("Hello, world!");
}
