use iced_x86::*;
use anyhow::Result;

pub fn nop_handler(_instr: Instruction) -> Result<()> {
    Ok(())
}