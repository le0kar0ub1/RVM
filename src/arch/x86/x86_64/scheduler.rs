use anyhow::Result;
use iced_x86::*;

use crate::arch;
use crate::mem;

pub fn scheduler(mem: mem::mem::Mem, img: *mut u8, ep: usize) -> Result<()> {
    let mut ep = ep;
    loop {
        let buffered = unsafe { 
            std::slice::from_raw_parts(ep as *const u8, 16)
        };
        let mut decoder = iced_x86::Decoder::new(64, &buffered, DecoderOptions::NONE);
        let instr = decoder.decode();
        println!("{:?}", instr);
        ep += instr.next_ip() as usize;
        arch::x86::shared::opcode_handler::handle_opcode(instr, mem)?;
    }
    Ok(())
}

pub fn init(mem: mem::mem::Mem, img: *mut u8, ep: usize) -> Result<()> {
    arch::x86::x86_64::cpu::init(mem.stack.get_addr() as u64, ep as u64);
    scheduler(mem, img, ep)?;
    Ok(())
}