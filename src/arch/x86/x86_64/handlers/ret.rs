use iced_x86::*;
use anyhow::{Result, anyhow};

use crate::arch::x86::x86_64::cpu;
use crate::mem::op;

pub fn ret_handler(instr: Instruction) -> Result<()> {
    let selfsz = instr.next_ip();
    match instr.code() {
        Code::Retnq => {
            let oldrip = op::safe_get64(cpu::get64(Register::RSP)? as usize)?;
            cpu::set64(Register::RSP, cpu::get64(Register::RSP)? + 8)?;
            cpu::set64(Register::RIP, oldrip - selfsz)
        },
        _ => Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr))),
    }?;
    Ok(())
}