use anyhow::Result;
use burst;

use crate::arch;
use crate::mem;

pub fn scheduler() {
    // let get = unsafe { 
    //     std::slice::from_raw_parts((ep + 4) as *const u8, 16)
    // };
    // println!("{:#X?}", get);
    // let instr = burst::x86::disassemble_64(&get, 0, 16);
    // println!("{:?}", instr);
    // println!("Hello, world!");
}

pub fn init(mem: mem::mem::Mem, img: *mut u8, ep: usize) -> Result<()> {
    let mut cpu = arch::x86::x86_64::cpu::Proc::new(mem.stack.get_addr() as u64, ep as u64)?;
    Ok(())
}