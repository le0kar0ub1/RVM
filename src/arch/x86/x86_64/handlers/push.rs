use iced_x86::*;
use anyhow::{Result, anyhow};

use crate::arch::x86::x86_64::cpu;

use crate::mem::op;

pub fn push_handler(instr: Instruction) -> Result<()> {
    let sub = match instr.code() {
        Code::Push_r16 => {
            op::safe_set16((cpu::get64(Register::RSP)? - 2) as usize, cpu::get16(instr.op_register(0))? as u16)?;
            Ok(2)
        },
        Code::Push_r32 => {
            op::safe_set32((cpu::get64(Register::RSP)? - 4) as usize, cpu::get32(instr.op_register(0))? as u32)?;
            Ok(4)
        },
        Code::Push_r64 => {
            op::safe_set64((cpu::get64(Register::RSP)? - 8) as usize, cpu::get64(instr.op_register(0))? as u64)?;
            Ok(8)
        },
        Code::Push_rm16 => {
            op::safe_set16((cpu::get64(Register::RSP)? - 2) as usize, op::safe_get16(instr.immediate(0) as usize)? as u16)?;
            Ok(2)
        },
        Code::Push_rm32 => {
            op::safe_set32((cpu::get64(Register::RSP)? - 4) as usize, op::safe_get32(instr.immediate(0) as usize)? as u32)?;
            Ok(4)
        },
        Code::Push_rm64 => {
            op::safe_set64((cpu::get64(Register::RSP)? - 8) as usize, op::safe_get64(instr.immediate(0) as usize)? as u64)?;
            Ok(8)
        },
        Code::Pushw_imm8 | Code::Pushq_imm8 | Code::Pushd_imm8 => {
            op::safe_set8((cpu::get64(Register::RSP)? - 1) as usize, instr.immediate(0) as u8)?;
            Ok(1)
        },
        Code::Push_imm16 => {
            op::safe_set16((cpu::get64(Register::RSP)? - 2) as usize, instr.immediate(0) as u16)?;
            Ok(2)
        },
        Code::Pushd_imm32 | Code::Pushq_imm32 => {
            op::safe_set32((cpu::get64(Register::RSP)? - 4) as usize, instr.immediate(0) as u32)?;
            Ok(4)
        }
        _ => Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr))),
    }?;
    cpu::set64(Register::RSP, cpu::get64(Register::RSP)? - sub as u64)?;
    Ok(())
}