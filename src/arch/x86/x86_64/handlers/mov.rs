use iced_x86::*;
use anyhow::{Result, anyhow};

use crate::arch::x86::x86_64::cpu;

use crate::mem::op;

pub fn mov_handler(instr: Instruction) -> Result<()> {
    match instr.code() {
        Code::Mov_rm64_r64 => {
            if instr.memory_base() != Register::None {
                op::safe_set64(cpu::get64(instr.memory_base())? as usize, cpu::get64(instr.op_register(1))?)
            } else if instr.memory_displacement() != 0 {
                op::safe_set64(instr.memory_displacement64() as usize, cpu::get64(instr.op_register(1))?)
            } else if instr.op_register(0) != Register::None {
                cpu::set64(instr.op_register(0), cpu::get64(instr.op_register(1))?)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Mov_rm32_r32 => { 
            if instr.memory_base() != Register::None {
                op::safe_set32(cpu::get64(instr.memory_base())? as usize, cpu::get32(instr.op_register(1))?)
            } else if instr.memory_displacement() != 0 {
                op::safe_set32(instr.memory_displacement64() as usize, cpu::get32(instr.op_register(1))?)
            } else if instr.op_register(0) != Register::None {
                cpu::set32(instr.op_register(0), cpu::get32(instr.op_register(1))?)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Mov_rm16_r16 => { 
            if instr.memory_base() != Register::None {
                op::safe_set16(cpu::get64(instr.memory_base())? as usize, cpu::get16(instr.op_register(1))?)
            } else if instr.memory_displacement() != 0 {
                op::safe_set16(instr.memory_displacement64() as usize, cpu::get16(instr.op_register(1))?)
            } else if instr.op_register(0) != Register::None {
                cpu::set16(instr.op_register(0), cpu::get16(instr.op_register(1))?)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Mov_rm8_r8   => { 
            if instr.memory_base() != Register::None {
                op::safe_set8(cpu::get64(instr.memory_base())? as usize, cpu::get8(instr.op_register(1))?)
            } else if instr.memory_displacement() != 0 {
                op::safe_set8(instr.memory_displacement64() as usize, cpu::get8(instr.op_register(1))?)
            } else if instr.op_register(0) != Register::None {
                cpu::set8(instr.op_register(0), cpu::get8(instr.op_register(1))?)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }

        Code::Mov_r64_rm64 => {
            if instr.memory_base() != Register::None {
                cpu::set64(instr.op_register(0), op::safe_get64(cpu::get64(instr.memory_base())? as usize)?)
            } else if instr.memory_displacement() != 0 {
                cpu::set64(instr.op_register(0), op::safe_get64(instr.memory_displacement64() as usize)?)
            } else if instr.op_register(0) != Register::None {
                cpu::set64(instr.op_register(0), cpu::get64(instr.op_register(1))?)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Mov_r32_rm32 => { 
            if instr.memory_base() != Register::None {
                cpu::set32(instr.op_register(0), op::safe_get32(cpu::get32(instr.memory_base())? as usize)?)
            } else if instr.memory_displacement() != 0 {
                cpu::set32(instr.op_register(0), op::safe_get32(instr.memory_displacement64() as usize)?)
            } else if instr.op_register(0) != Register::None {
                cpu::set32(instr.op_register(0), cpu::get32(instr.op_register(1))?)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Mov_r16_rm16 => { 
            if instr.memory_base() != Register::None {
                cpu::set16(instr.op_register(0), op::safe_get16(cpu::get16(instr.memory_base())? as usize)?)
            } else if instr.memory_displacement() != 0 {
                cpu::set16(instr.op_register(0), op::safe_get16(instr.memory_displacement() as usize)?)
            } else if instr.op_register(0) != Register::None {
                cpu::set16(instr.op_register(0), cpu::get16(instr.op_register(1))?)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Mov_r8_rm8   => { 
            if instr.memory_base() != Register::None {
                cpu::set8(instr.op_register(0), op::safe_get8(cpu::get8(instr.memory_base())? as usize)?)
            } else if instr.memory_displacement() != 0 {
                cpu::set8(instr.op_register(0), op::safe_get8(instr.memory_displacement() as usize)?)
            } else if instr.op_register(0) != Register::None {
                cpu::set8(instr.op_register(0), cpu::get8(instr.op_register(1))?)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }

        Code::Mov_rm64_imm32 => {
            if instr.memory_base() != Register::None {
                op::safe_set64(cpu::get64(instr.memory_base())? as usize, instr.immediate(1) as u64)
            } else if instr.memory_displacement() != 0 {
                op::safe_set64(instr.memory_displacement64() as usize, instr.immediate(1) as u64)
            } else if instr.op_register(0) != Register::None {
                cpu::set64(instr.op_register(0), instr.immediate(1) as u64)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Mov_rm32_imm32 => { 
            if instr.memory_base() != Register::None {
                op::safe_set32(cpu::get64(instr.memory_base())? as usize, instr.immediate(1) as u32)
            } else if instr.memory_displacement() != 0 {
                op::safe_set32(instr.memory_displacement64() as usize, instr.immediate(1) as u32)
            } else if instr.op_register(0) != Register::None {
                cpu::set32(instr.op_register(0), instr.immediate(1) as u32)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Mov_rm16_imm16 => { 
            if instr.memory_base() != Register::None {
                op::safe_set16(cpu::get64(instr.memory_base())? as usize, instr.immediate(1) as u16)
            } else if instr.memory_displacement() != 0 {
                op::safe_set16(instr.memory_displacement64() as usize, instr.immediate(1) as u16)
            } else if instr.op_register(0) != Register::None {
                cpu::set16(instr.op_register(0), instr.immediate(1) as u16)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Mov_rm8_imm8   => { 
            if instr.memory_base() != Register::None {
                op::safe_set8(cpu::get64(instr.memory_base())? as usize, instr.immediate(0) as u8)
            } else if instr.memory_displacement() != 0 {
                op::safe_set8(instr.memory_displacement64() as usize, instr.immediate(0) as u8)
            } else if instr.op_register(0) != Register::None {
                cpu::set8(instr.op_register(0), instr.immediate(0) as u8)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }

        Code::Mov_r64_imm64 => {
            if instr.memory_base() != Register::None {
                cpu::set64(instr.op_register(0), instr.immediate(1) as u64)
            } else if instr.memory_displacement() != 0 {
                cpu::set64(instr.op_register(0), instr.immediate(1) as u64)
            } else if instr.op_register(0) != Register::None {
                cpu::set64(instr.op_register(0), instr.immediate(1) as u64)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Mov_r32_imm32 => { 
            if instr.memory_base() != Register::None {
                cpu::set64(instr.op_register(0).full_register(), instr.immediate(1) as u64)
            } else if instr.memory_displacement() != 0 {
                cpu::set64(instr.op_register(0).full_register(), instr.immediate(1) as u64)
            } else if instr.op_register(0) != Register::None {
                cpu::set64(instr.op_register(0).full_register(), instr.immediate(1) as u64)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Mov_r16_imm16 => { 
            if instr.memory_base() != Register::None {
                cpu::set16(instr.op_register(0), instr.immediate(1) as u16)
            } else if instr.memory_displacement() != 0 {
                cpu::set16(instr.op_register(0), instr.immediate(1) as u16)
            } else if instr.op_register(0) != Register::None {
                cpu::set16(instr.op_register(0), instr.immediate(1) as u16)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Mov_r8_imm8   => { 
            if instr.memory_base() != Register::None {
                cpu::set8(instr.op_register(0), instr.immediate(1) as u8)
            } else if instr.memory_displacement() != 0 {
                cpu::set8(instr.op_register(0), instr.immediate(1) as u8)
            } else if instr.op_register(0) != Register::None {
                cpu::set8(instr.op_register(0), instr.immediate(1) as u8)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }

        _ => Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr))),
    }?;
    Ok(())
}