use iced_x86::*;
use anyhow::Result;

use crate::arch::x86::shared::cpu;

use crate::mem::hit;

pub fn mov_handler(instr: Instruction) -> Result<()> {
    println!("{:?}", instr.code());
    println!("{:?} {:?}", instr.op_kind(0), instr.op_kind(1));
    match instr.code() {
        Code::Mov_rm64_r64 => cpu::set64_register(instr.op_register(0), cpu::get64_register(instr.op_register(1))?),
        Code::Mov_rm32_r32 => cpu::set32_register(instr.op_register(0), cpu::get32_register(instr.op_register(1))?),
        Code::Mov_rm16_r16 => cpu::set16_register(instr.op_register(0), cpu::get16_register(instr.op_register(1))?),
        Code::Mov_rm8_r8 => cpu::set8_register(instr.op_register(0), cpu::get8_register(instr.op_register(1))?),
        Code::Mov_rm64_imm32 => cpu::set64_register(instr.op_register(0), instr.immediate(1)),
        Code::Mov_rm32_imm32 => cpu::set32_register(instr.op_register(0), instr.immediate(1) as u32),
        Code::Mov_rm16_imm16 => cpu::set16_register(instr.op_register(0), instr.immediate(1) as u16), 
        Code::Mov_rm8_imm8 => cpu::set8_register(instr.op_register(0), instr.immediate(1) as u8),

        Code::Mov_r64_rm64 => 
        _ => Err(anyhow::anyhow!(format!("Invalid mov instruction:\n {:?}", instr)))
    }?;
    Ok(())
}