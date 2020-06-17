use std::env;

use anyhow::Result;
use burst;

mod loader;
mod arch;
mod mem;

fn launch(file: &String) -> Result<()> {
    let mut elfimg = loader::elf::load::ElfImg::new(file)?;
    let ep = elfimg.load()?;
    let cpu = arch::x86::x86_64::cpu::Proc::new(0x10000, ep as u64)?;
    let get = unsafe { 
        std::slice::from_raw_parts((ep + 4) as *const u8, 16)
    };
    let mem = mem::mem::Mem::new(0x10000);
    println!("{:#X?}", get);
    let instr = burst::x86::disassemble_64(&get, 0, 16);
    println!("{:?}", instr);
    println!("Hello, world!");
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(anyhow::anyhow!("Usage: \"cargo run $BINARY\""))
    }
    launch(&args[1])?;
    Ok(())
}
