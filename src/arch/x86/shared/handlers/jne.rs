use iced_x86::*;
use anyhow::{Result, anyhow};

use crate::arch::x86::shared::cpu;

pub fn jne_handler(instr: Instruction) -> Result<()> {
    if cpu::supervis_get_flags_register() & cpu::FlagRegister::ZF as u64 == 0 {
        let selfsz = instr.next_ip();
        match instr.code() {
            Code::Jne_rel32_64 | Code::Jne_rel8_64 | Code::Jne_rel32_32 | Code::Jne_rel8_32 | Code::Jne_rel8_16 => {
                cpu::set64(Register::RIP, (cpu::get64(Register::RIP)? as i64 + instr.memory_address64() as i64 - selfsz as i64) as u64)
            }
            _ => Err(anyhow!(format!("Invalid operand/format:\n{:?}", instr))),
        }?;
    }
    Ok(())
}