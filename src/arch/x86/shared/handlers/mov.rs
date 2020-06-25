use iced_x86::*;
use anyhow::Result;

use crate::arch::x86::shared::cpu;

use crate::mem::op;

pub fn mov_handler(instr: Instruction) -> Result<()> {
    // println!("{:?}", instr.code());
    match instr.code() {
        Code::Mov_rm64_r64 => op::safe_set64(cpu::get64_register(instr.memory_base())? as usize, cpu::get64_register(instr.op_register(1))?),
        Code::Mov_rm32_r32 => op::safe_set32(cpu::get64_register(instr.memory_base())? as usize, cpu::get32_register(instr.op_register(1))?),
        Code::Mov_rm16_r16 => op::safe_set16(cpu::get64_register(instr.memory_base())? as usize, cpu::get16_register(instr.op_register(1))?),
        Code::Mov_rm8_r8   => op::safe_set8(cpu::get64_register(instr.memory_base())? as usize,  cpu::get8_register(instr.op_register(1))?),

        Code::Mov_r64_rm64 => cpu::set64_register(instr.op_register(0), op::safe_get64(cpu::get64_register(instr.memory_base())? as usize)?),
        Code::Mov_r32_rm32 => cpu::set32_register(instr.op_register(0), op::safe_get32(cpu::get64_register(instr.memory_base())? as usize)?),
        Code::Mov_r16_rm16 => cpu::set16_register(instr.op_register(0), op::safe_get16(cpu::get64_register(instr.memory_base())? as usize)?),
        Code::Mov_r8_rm8   => cpu::set8_register(instr.op_register(0), op::safe_get8(cpu::get64_register(instr.memory_base())? as usize)?),

        Code::Mov_rm64_imm32 => op::safe_set64(cpu::get64_register(instr.memory_base())? as usize, instr.immediate(1)),
        Code::Mov_rm32_imm32 => op::safe_set32(cpu::get64_register(instr.memory_base())? as usize, instr.immediate(1) as u32),
        Code::Mov_rm16_imm16 => op::safe_set16(cpu::get64_register(instr.memory_base())? as usize, instr.immediate(1) as u16), 
        Code::Mov_rm8_imm8 => op::safe_set8(cpu::get64_register(instr.memory_base())? as usize, instr.immediate(1) as u8),

        Code::Mov_r64_imm64 => cpu::set64_register(instr.op_register(0), instr.immediate(1)),
        Code::Mov_r32_imm32 => cpu::set32_register(instr.op_register(0), instr.immediate(1) as u32),
        Code::Mov_r16_imm16 => cpu::set16_register(instr.op_register(0), instr.immediate(1) as u16),
        Code::Mov_r8_imm8 => cpu::set8_register(instr.op_register(0), instr.immediate(1) as u8),

        _ => Err(anyhow::anyhow!(format!("Invalid operand/format:\n{:?}", instr))),
    }?;
    Ok(())
}