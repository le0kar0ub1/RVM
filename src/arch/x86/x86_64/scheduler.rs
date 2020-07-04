use anyhow::{Result, anyhow};
use iced_x86::*;

use crate as root;
use crate::arch;
use crate::mem;

pub fn scheduler(_img: *mut u8, ep: usize) -> Result<()> {
    let xseg = mem::mem::segment_get(ep)?;
    let mut rip = ep;
    if (xseg.flags as u32 & mem::comp::segments::SegmentFlag::X as u32) == 0 {
        return Err(anyhow!("Entry point hit a non-executable segment"))
    }
    while xseg.addr <= rip && rip <= xseg.addr + xseg.size && xseg.dirty == false {
        let rd = match (xseg.addr + xseg.size) - rip {
            0..=16 => ((xseg.addr + xseg.size) - rip),
            _ => 16
        };
        let buffered = unsafe { 
            std::slice::from_raw_parts(rip as *const u8, rd)
        };
        let mut decoder = iced_x86::Decoder::new(64, &buffered, DecoderOptions::NONE);
        let instr = decoder.decode();
        println!("{:?}", instr.code());
        arch::x86::x86_64::opcode_handler::handle_opcode(instr)?;
        arch::x86::shared::cpu::set64(Register::RIP, arch::x86::shared::cpu::get64(Register::RIP)? + instr.next_ip() as u64)?;
        rip = arch::x86::shared::cpu::get64(Register::RIP)? as usize;
    }
    Ok(())
}

pub fn init(img: *mut u8, ep: usize) -> Result<()> {
    arch::x86::shared::cpu::init(mem::mem::stack_get().get_addr() as u64, ep as u64);
    scheduler(img, ep)?;
    Ok(())
}

pub fn arch_exit(exitcode: i32) {
    root::supervisor_exit(exitcode);
}