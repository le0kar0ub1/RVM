use std::env;

use anyhow::Result;

mod loader;
mod arch;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => (),
        _ => panic!("Usage: \"cargo run $BINARY\"")
    }
    let mut elfimg = loader::elf::load::ElfImg::new(&args[1])?;
    elfimg.load()?;
    // let proc = arch::x86::x86_64::cpu::init(0, 0);
    println!("Hello, world!");
    Ok(())
}
