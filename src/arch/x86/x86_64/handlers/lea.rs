use iced_x86::*;
use anyhow::{Result, anyhow};

use crate::arch::x86::x86_64::cpu;
use crate::mem::mem;

pub fn lea_handler(instr: Instruction) -> Result<()> {
    match instr.code() {
        Code::Lea_r64_m => {
            cpu::set64(instr.op_register(0), mem::tovirt(instr.memory_displacement64() as usize) as u64)
        },
        _ => Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr))),
    }?;
    Ok(())
}