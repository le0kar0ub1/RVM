use iced_x86::*;
use anyhow::{Result, anyhow};

use crate::arch::x86::shared::cpu;

use crate::mem::op;
use crate::mem::mem;

pub fn cmp_handler(instr: Instruction) -> Result<()> {
    let mut flg = cpu::supervis_get_flags_register();
    // match instr.code() {
    //     Code::Cmp_r8_rm8 => {
    //         if instr.memory_base() != Register::None {
    //             op::safe_set8(cpu::get64_register(instr.memory_base())? as usize, 
    //                 op::safe_get8(cpu::get64_register(instr.memory_base())? as usize)? & cpu::get8_register(instr.op_register(1))?)
    //         } else if instr.memory_displacement() != 0 {
    //             op::safe_set8(instr.memory_displacement64() as usize,
    //                 op::safe_get8(instr.memory_displacement64() as usize)? & cpu::get8_register(instr.op_register(1))?)
    //         } else if instr.op_register(0) != Register::None {
    //             cpu::set8_register(instr.op_register(0), cpu::get8_register(instr.op_register(0))? & cpu::get8_register(instr.op_register(1))?)
    //         } else {
    //             Err(anyhow!("Invalid opcode"))
    //         }
    //     }
    //     Code::Cmp_r16_rm16 => {}
    //     Code::Cmp_r32_rm32 => {}
    //     Code::Cmp_r64_rm64 => {}
    // }
    cpu::supervis_set_flags_register(flg);
    Ok(())
}