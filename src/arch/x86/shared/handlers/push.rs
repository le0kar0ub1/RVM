use iced_x86::*;
use anyhow::Result;

use crate::arch::x86::shared::cpu;

use crate::mem::op;

pub fn push_handler(instr: Instruction) -> Result<()> {
    let sub = match instr.code() {
        Code::Push_r16 => {
            op::safe_set16(cpu::get64_register(Register::RSP)? as usize, cpu::get16_register(instr.op_register(0))? as u16)?;
            Ok(2)
        },
        Code::Push_r32 => {
            op::safe_set32(cpu::get64_register(Register::RSP)? as usize, cpu::get32_register(instr.op_register(0))? as u32)?;
            Ok(4)
        },
        Code::Push_r64 => {
            op::safe_set64(cpu::get64_register(Register::RSP)? as usize, cpu::get64_register(instr.op_register(0))? as u64)?;
            Ok(8)
        },
        Code::Push_rm16 => {
            op::safe_set16(cpu::get64_register(Register::RSP)? as usize, op::safe_get16(cpu::get64_register(instr.op_register(0))? as usize)? as u16)?;
            Ok(2)
        },
        Code::Push_rm32 => {
            op::safe_set32(cpu::get64_register(Register::RSP)? as usize, op::safe_get32(cpu::get64_register(instr.op_register(0))? as usize)? as u32)?;
            Ok(4)
        },
        Code::Push_rm64 => {
            op::safe_set64(cpu::get64_register(Register::RSP)? as usize, op::safe_get64(cpu::get64_register(instr.op_register(0))? as usize)? as u64)?;
            Ok(8)
        },
        Code::Pushw_imm8 | Code::Pushq_imm8 | Code::Pushd_imm8 => {
            op::safe_set8(cpu::get64_register(Register::RSP)? as usize, instr.immediate(0) as u8)?;
            Ok(1)
        },
        Code::Push_imm16 => {
            op::safe_set16(cpu::get64_register(Register::RSP)? as usize, instr.immediate(0) as u16)?;
            Ok(2)
        },
        Code::Pushd_imm32 | Code::Pushq_imm32 => {
            op::safe_set32(cpu::get64_register(Register::RSP)? as usize, instr.immediate(0) as u32)?;
            Ok(4)
        }
        _ => Err(anyhow::anyhow!(format!("Invalid operand/format:\n{:?}", instr))),
    }?;
    cpu::set64_register(Register::RSP, cpu::get64_register(Register::RSP)? - sub as u64)?;
    Ok(())
}