#![feature(llvm_asm)]

use std;

extern crate goblin;

use anyhow::{Result, anyhow};

mod loader;
mod arch;
mod mem;

fn init(file: &String) -> Result<()> {
    let mut elfimg = loader::elf::load::ElfImg::new(file)?;
    let ep = elfimg.load()?;
    arch::x86::x86_64::scheduler::init(elfimg.img, ep)?;
    Ok(())
}

fn supervisor_exit(exitcode: i32) -> ! {
    println!("Program exit with code: {}", exitcode);
    std::process::exit(0x0);
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: \"cargo run $BINARY\""))
    }
    init(&args[1])?;
    Ok(())
}
