use std::env;

extern crate goblin;

use anyhow::Result;

mod loader;
mod arch;
mod mem;

fn init(file: &String) -> Result<()> {
    let mut elfimg = loader::elf::load::ElfImg::new(file)?;
    let ep = elfimg.load()?;
    let segments = elfimg.load_segments()?;
    let mem = mem::mem::Mem::new(0x100000, segments)?;
    let archres = elfimg.load_get_arch()?;
    match archres {
        goblin::elf::header::EM_X86_64 => arch::x86::x86_64::scheduler::init(mem, elfimg.img, ep),
        _ => Err(anyhow::anyhow!("Unhandled architecture"))
    }?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(anyhow::anyhow!("Usage: \"cargo run $BINARY\""))
    }
    init(&args[1])?;
    Ok(())
}
