use iced_x86::*;
use anyhow::{Result, anyhow};

use crate::arch::x86::x86_64::cpu;
use crate::arch::x86::x86_64::alu;
use crate::mem::op;

pub fn inc_handler(instr: Instruction) -> Result<()> {
    match instr.code() {
        Code::Inc_r16 =>
            cpu::set16(instr.op_register(0), alu::add(cpu::get16(instr.op_register(0))? as u64, 1 as u64, 16, 16)? as u16),
        Code::Inc_r32 =>
            cpu::set32(instr.op_register(0), alu::add(cpu::get32(instr.op_register(0))? as u64, 1 as u64, 16, 16)? as u32),
        Code::Inc_rm8 => {
            if instr.memory_base() != Register::None {
                op::safe_set8(cpu::get64(instr.memory_base())? as usize, alu::add(op::safe_get8(cpu::get64(instr.memory_base())? as usize)? as u64, 1 as u64, 8, 8)? as u8)
            } else if instr.op_register(0) != Register::None {
                cpu::set8(instr.op_register(0), alu::add(cpu::get8(instr.op_register(0))? as u64, 1 as u64, 8, 8)? as u8)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Inc_rm16 => {
            if instr.memory_base() != Register::None {
                op::safe_set16(cpu::get64(instr.memory_base())? as usize, alu::add(op::safe_get16(cpu::get64(instr.memory_base())? as usize)? as u64, 1 as u64, 16, 16)? as u16)
            } else if instr.op_register(0) != Register::None {
                cpu::set16(instr.op_register(0), alu::add(cpu::get16(instr.op_register(0))? as u64, 1 as u64, 16, 16)? as u16)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Inc_rm32 => {
            if instr.memory_base() != Register::None {
                op::safe_set32(cpu::get64(instr.memory_base())? as usize, alu::add(op::safe_get32(cpu::get64(instr.memory_base())? as usize)? as u64, 1 as u64, 32, 32)? as u32)
            } else if instr.op_register(0) != Register::None {
                cpu::set32(instr.op_register(0), alu::add(cpu::get32(instr.op_register(0))? as u64, 1 as u64, 32, 32)? as u32)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        Code::Inc_rm64 => {
            if instr.memory_base() != Register::None {
                op::safe_set64(cpu::get64(instr.memory_base())? as usize, alu::add(op::safe_get64(cpu::get64(instr.memory_base())? as usize)? as u64, 1 as u64, 64, 64)? as u64)
            } else if instr.op_register(0) != Register::None {
                cpu::set64(instr.op_register(0), alu::add(cpu::get64(instr.op_register(0))? as u64, 1 as u64, 64, 64)? as u64)
            } else {
                Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
            }
        }
        _ => Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr)))
    }?;
    Ok(())
}