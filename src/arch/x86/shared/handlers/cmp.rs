use iced_x86::*;
use anyhow::{Result, anyhow};

use crate::arch::x86::shared::cpu;

use crate::mem::op;
use crate::mem::mem;

pub fn cmp_handler(instr: Instruction) -> Result<()> {
    Ok(())
}