use iced_x86::*;
use anyhow::{Result, anyhow};

use crate::arch::x86::x86_64::cpu;

use crate::mem::op;
use crate::mem::mem;

pub fn jmp_handler(instr: Instruction) -> Result<()> {
    let selfsz = instr.next_ip();
    match instr.code() {
        Code::Jmp_rm64 => match instr.op_register(0) {
            Register::None => cpu::set64(Register::RIP, mem::iftranslation(op::safe_get64(instr.memory_address64() as usize)? as usize) as u64 - selfsz),
            _ => cpu::set64(Register::RIP, mem::iftranslation(cpu::get64(instr.op_register(0))? as usize) as u64 - selfsz),
        }
        Code::Jmp_ptr1616 | Code::Jmp_ptr1632 =>
            cpu::set64(Register::RIP, mem::iftranslation(instr.memory_address64() as usize) as u64 - selfsz),
        Code::Jmp_m1664 | Code::Jmp_m1632 | Code::Jmp_m1616 =>
            cpu::set64(Register::RIP, mem::iftranslation(op::safe_get64(instr.memory_address64() as usize)? as usize) as u64 - selfsz),
        Code::Jmp_rel32_64 | Code::Jmp_rel8_64 => {
            cpu::set64(Register::RIP, (cpu::get64(Register::RIP)? as i64 + instr.memory_address64() as i64 - selfsz as i64) as u64)
        }
        _ => Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr))),
    }?;
    Ok(())
}