use iced_x86::*;
use anyhow::{Result, anyhow};

use crate::arch::x86::shared::cpu;

use crate::mem::op;
use crate::mem::mem;

pub fn cmp_handler(instr: Instruction) -> Result<()> {
    let mut flg = cpu::supervis_get_flags_register();
    let cmp = match instr.code() {
        Code::Cmp_r8_rm8 => {
            if instr.memory_base() != Register::None {
                Ok([op::safe_get64(cpu::get64(instr.memory_base())? as usize)?, cpu::get8(instr.op_register(1))? as u64, 8])
            } else if instr.memory_displacement() != 0 {
                Ok([op::safe_get64(instr.memory_displacement64() as usize)?, cpu::get8(instr.op_register(1))? as u64, 8])
            } else if instr.op_register(0) != Register::None {
                Ok([cpu::get64(instr.op_register(0))?, cpu::get8(instr.op_register(1))? as u64, 8])
            } else {
                Err(anyhow!("Invalid opcode"))
            }
        },
        Code::Cmp_r16_rm16 => {
            if instr.memory_base() != Register::None {
                Ok([op::safe_get64(cpu::get64(instr.memory_base())? as usize)?, cpu::get16(instr.op_register(1))? as u64, 8])
            } else if instr.memory_displacement() != 0 {
                Ok([op::safe_get64(instr.memory_displacement64() as usize)?, cpu::get16(instr.op_register(1))? as u64, 8])
            } else if instr.op_register(0) != Register::None {
                Ok([cpu::get64(instr.op_register(0))?, cpu::get16(instr.op_register(1))? as u64, 8])
            } else {
                Err(anyhow!("Invalid opcode"))
            }
        }
        Code::Cmp_r32_rm32 => {
            if instr.memory_base() != Register::None {
                Ok([op::safe_get64(cpu::get64(instr.memory_base())? as usize)?, cpu::get32(instr.op_register(1))? as u64, 8])
            } else if instr.memory_displacement() != 0 {
                Ok([op::safe_get64(instr.memory_displacement64() as usize)?, cpu::get32(instr.op_register(1))? as u64, 8])
            } else if instr.op_register(0) != Register::None {
                Ok([cpu::get64(instr.op_register(0))?, cpu::get32(instr.op_register(1))? as u64, 8])
            } else {
                Err(anyhow!("Invalid opcode"))
            }
        }
        Code::Cmp_r64_rm64 => {
            if instr.memory_base() != Register::None {
                Ok([op::safe_get64(cpu::get64(instr.memory_base())? as usize)?, cpu::get64(instr.op_register(1))? as u64, 8])
            } else if instr.memory_displacement() != 0 {
                Ok([op::safe_get64(instr.memory_displacement64() as usize)?, cpu::get64(instr.op_register(1))? as u64, 8])
            } else if instr.op_register(0) != Register::None {
                Ok([cpu::get64(instr.op_register(0))?, cpu::get64(instr.op_register(1))? as u64, 8])
            } else {
                Err(anyhow!("Invalid opcode"))
            }
        }

        Code::Cmp_rm8_imm8 => {
            if instr.memory_base() != Register::None {
                Ok([op::safe_get64(cpu::get64(instr.memory_base())? as usize)?, cpu::get64(instr.op_register(1))? as u64, 8])
            } else if instr.memory_displacement() != 0 {
                Ok([op::safe_get64(instr.memory_displacement64() as usize)?, cpu::get64(instr.op_register(1))? as u64, 8])
            } else if instr.op_register(0) != Register::None {
                Ok([cpu::get64(instr.op_register(0))?, cpu::get64(instr.op_register(1))? as u64, 8])
            } else {
                Err(anyhow!("Invalid opcode"))
            }
        }
        _ => Err(anyhow!("Invalid opcode"))
    }?;
    println!("{:?}", cmp);
    cpu::supervis_set_flags_register(flg);
    Ok(())
}