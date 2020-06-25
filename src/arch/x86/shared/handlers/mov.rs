use iced_x86::*;
use anyhow::Result;

use crate::arch::x86::shared::cpu;

use crate::mem::op;

pub fn mov_handler(instr: Instruction) -> Result<()> {
    println!("{:?}", instr.code());
    println!("{:?}", instr);
    match instr.code() {
        Code::Mov_rm64_r64 => cpu::set64_register(instr.op_register(0), cpu::get64_register(instr.op_register(1))?),
        Code::Mov_rm32_r32 => cpu::set32_register(instr.op_register(0), cpu::get32_register(instr.op_register(1))?),
        Code::Mov_rm16_r16 => cpu::set16_register(instr.op_register(0), cpu::get16_register(instr.op_register(1))?),
        Code::Mov_rm8_r8   => cpu::set8_register(instr.op_register(0),  cpu::get8_register(instr.op_register(1))?),
        
        Code::Mov_rm64_imm32 => op::safe_set64(cpu::get64_register(instr.memory_base())? as usize, instr.immediate(1)),
        Code::Mov_rm32_imm32 => op::safe_set32(cpu::get64_register(instr.memory_base())? as usize, instr.immediate(1) as u32),
        Code::Mov_rm16_imm16 => op::safe_set16(cpu::get64_register(instr.memory_base())? as usize, instr.immediate(1) as u16), 
        Code::Mov_rm8_imm8 => op::safe_set8(cpu::get64_register(instr.memory_base())? as usize, instr.immediate(1) as u8),

        // Code::Mov_r64_rm64 => op::safe_set64(instr.)

        Code::Mov_r64_imm64 => cpu::set64_register(instr.op_register(0), instr.immediate(1)),
        Code::Mov_r32_imm32 => cpu::set32_register(instr.op_register(0), instr.immediate(1) as u32),
        Code::Mov_r16_imm16 => cpu::set16_register(instr.op_register(0), instr.immediate(1) as u16),
        Code::Mov_r8_imm8 => cpu::set8_register(instr.op_register(0), instr.immediate(1) as u8),
        _ => Err(anyhow::anyhow!(format!("Invalid operand/format:\n {:?}", instr))),
    }?;
    Ok(())
}