use iced_x86::*;
use anyhow::Result;

use crate::arch::x86::shared::cpu;

use crate::mem::op;

pub fn syscall_handler(instr: Instruction) -> Result<()> {
    println!("{:?}", instr);
    cpu::dump_user_base_regs();
    Ok(())
}