use iced_x86::*;
use anyhow::Result;

use crate::arch::x86::shared::cpu;

use crate::mem::op;

pub fn xor_handler(instr: Instruction) -> Result<()> {
    cpu::dump_user_base_regs();
    println!("{:?}", instr.code());
    match instr.code() {
        Code::Xor_rm64_r64 => match instr.memory_base() == Register::None { 
            true  => cpu::set64_register(instr.op_register(0), cpu::get64_register(instr.op_register(0))? ^ cpu::get64_register(instr.op_register(1))?),
            false => op::safe_set64(cpu::get64_register(instr.memory_base())? as usize, 
                op::safe_get64(cpu::get64_register(instr.memory_base())? as usize)? ^ cpu::get64_register(instr.op_register(1))?),
        }
        Code::Xor_rm32_r32 => match instr.memory_base() == Register::None {
            true  => cpu::set32_register(instr.op_register(0), cpu::get32_register(instr.op_register(0))? ^ cpu::get32_register(instr.op_register(1))?),
            false => op::safe_set32(cpu::get32_register(instr.memory_base())? as usize, 
                op::safe_get32(cpu::get32_register(instr.memory_base())? as usize)? ^ cpu::get32_register(instr.op_register(1))?),
        }
        Code::Xor_rm16_r16 => cpu::set16_register(instr.op_register(0), cpu::get16_register(instr.op_register(0))? ^ cpu::get16_register(instr.op_register(1))?),
        Code::Xor_rm8_r8   => cpu::set8_register(instr.op_register(0), cpu::get8_register(instr.op_register(0))? ^ cpu::get8_register(instr.op_register(1))?),

        Code::Xor_r64_rm64 => op::safe_set64(cpu::get64_register(instr.memory_base())? as usize,
            (cpu::get64_register(instr.op_register(0))? ^ op::safe_get64(cpu::get64_register(instr.memory_base())? as usize )?) as u64),
        Code::Xor_r32_rm32 => op::safe_set32(cpu::get64_register(instr.memory_base())? as usize,
            (cpu::get32_register(instr.op_register(0))? ^ op::safe_get32(cpu::get64_register(instr.memory_base())? as usize )?) as u32)?,
        Code::Xor_r16_rm16 => op::safe_set16(cpu::get64_register(instr.memory_base())? as usize,
            (cpu::get16_register(instr.op_register(0))? ^ op::safe_get16(cpu::get64_register(instr.memory_base())? as usize )?) as u16)?,
        Code::Xor_r8_rm8   => op::safe_set8(cpu::get64_register(instr.memory_base())? as usize,
            (cpu::get8_register(instr.op_register(0))? ^ op::safe_get8(cpu::get64_register(instr.memory_base())? as usize )?) as u8)?,

        Code::Xor_rm64_imm32 => op::safe_set64(cpu::get64_register(instr.memory_base())? as usize, instr.immediate(1)),
        Code::Xor_rm32_imm32 => op::safe_set32(cpu::get64_register(instr.memory_base())? as usize, instr.immediate(1) as u32),
        Code::Xor_rm16_imm16 => op::safe_set16(cpu::get64_register(instr.memory_base())? as usize, instr.immediate(1) as u16),
        Code::Xor_rm8_imm8   => op::safe_set8(cpu::get64_register(instr.memory_base())? as usize, instr.immediate(1) as u8),

        _ => Err(anyhow::anyhow!(format!("Invalid operand/format:\n{:?}", instr))),
    }?;
    cpu::dump_user_base_regs();
    Ok(())
}