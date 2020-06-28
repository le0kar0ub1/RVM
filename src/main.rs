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
    let segments = elfimg.load_segments()?;
    mem::mem::init(0x100000, segments, elfimg.img as usize, 0, elfimg.imgsz as usize)?;
    let archtgt = elfimg.load_get_arch()?;
    match archtgt {
        goblin::elf::header::EM_X86_64 => arch::x86::x86_64::scheduler::init(elfimg.img, ep),
        _ => Err(anyhow!("Unhandled architecture"))
    }?;
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
