use anyhow::Result;
use iced_x86::*;

use crate::arch;
use crate::mem;

pub fn scheduler(img: *mut u8, ep: usize) -> Result<()> {
    let mut ep = ep;
    loop {
        let buffered = unsafe { 
            std::slice::from_raw_parts(ep as *const u8, 16)
        };
        let mut decoder = iced_x86::Decoder::new(64, &buffered, DecoderOptions::NONE);
        let instr = decoder.decode();
        println!("{:?}", instr);
        ep += instr.next_ip() as usize;
        arch::x86::shared::opcode_handler::handle_opcode(instr)?;
    }
    Ok(())
}

pub fn init(img: *mut u8, ep: usize) -> Result<()> {
    arch::x86::x86_64::cpu::init(mem::mem::stack_get().get_addr() as u64, ep as u64);
    scheduler(img, ep)?;
    Ok(())
}