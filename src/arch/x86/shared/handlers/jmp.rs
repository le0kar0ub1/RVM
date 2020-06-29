use iced_x86::*;
use anyhow::Result;

use crate::arch::x86::shared::cpu;

use crate::mem::op;

pub fn jmp_handler(instr: Instruction) -> Result<()> {
    println!("{:?}", instr);
    let selfsz = instr.next_ip();
    match instr.code() {
        Code::Jmp_rm64 => match instr.op0_register() {
            Register::None => cpu::set64_register(Register::RIP, op::safe_get64(instr.memory_displacement64() as usize)? - selfsz),
            _ => cpu::set64_register(Register::RIP, cpu::get64_register(instr.op0_register())? - selfsz)
        }
        Code::Jmp_ptr1616 | Code::Jmp_ptr1632 => {
            cpu::set64_register(Register::RIP, instr.immediate(0) - selfsz),
        }
        Code::Jmp_m1664 | Code::Jmp_1632 | Code::Jmp_m1616 => {
            cpu::set64_register(Register::RIP, op::safe_get64(instr.memory_displacement64() as usize)? - selfsz),
        }
        // Code::Jmp_rel1
        _ => Ok(()),
    }?;
    Ok(())
}