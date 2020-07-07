use iced_x86::*;
use anyhow::{Result, anyhow};

use crate::arch::x86::x86_64::cpu;
use crate::mem::mem;
use crate::mem::op;

pub fn call_handler(instr: Instruction) -> Result<()> {
    let selfsz = instr.next_ip();
    let oldrip = cpu::get64(Register::RIP)? + selfsz;
    match instr.code() {
        Code::Call_rel16 | Code::Call_rel32_32 | Code::Call_rel32_64 => {
            cpu::set64(Register::RIP, (cpu::get64(Register::RIP)? as i64 + instr.memory_address64() as i64) as u64 - selfsz)
        }
        Code::Call_rm64 => {
            cpu::set64(Register::RIP, mem::is_segment_executable((cpu::get64(instr.op_register(0))?) as usize, 1 as usize)? as u64 - selfsz)
        }
        _ => Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr))),
    }?;
    cpu::set64(Register::RSP, cpu::get64(Register::RSP)? - 8)?;
    op::safe_set64(cpu::get64(Register::RSP)? as usize, oldrip)?;
    Ok(())
}