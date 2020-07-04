use iced_x86::*;
use anyhow::{Result, anyhow};

use crate::arch::x86::shared::cpu;

use crate::mem::op;

pub fn pop_handler(instr: Instruction) -> Result<()> {
    let add = match instr.code() {
        Code::Pop_r16 => {
            cpu::set16(instr.op_register(0), op::safe_get16(cpu::get64(Register::RSP)? as usize)?)?;
            Ok(2)
        },
        Code::Pop_r32 => {
            cpu::set32(instr.op_register(0), op::safe_get32(cpu::get64(Register::RSP)? as usize)?)?;
            Ok(4)
        },
        Code::Pop_r64 => {
            cpu::set64(instr.op_register(0), op::safe_get64(cpu::get64(Register::RSP)? as usize)?)?;
            Ok(8)
        },
        Code::Pop_rm16 => {
            op::safe_set16(instr.immediate(0) as usize, op::safe_get16(cpu::get64(Register::RSP)? as usize)? as u16)?;
            Ok(2)
        },
        Code::Pop_rm32 => {
            op::safe_set32(instr.immediate(0) as usize, op::safe_get32(cpu::get64(Register::RSP)? as usize)? as u32)?;
            Ok(4)
        },
        Code::Pop_rm64 => {
            op::safe_set64(instr.immediate(0) as usize, op::safe_get64(cpu::get64(Register::RSP)? as usize)? as u64)?;
            Ok(8)
        },
        _ => Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr))),
    }?;
    cpu::set64(Register::RSP, cpu::get64(Register::RSP)? + add as u64)?;
    Ok(())
}