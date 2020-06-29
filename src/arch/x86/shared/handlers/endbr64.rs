use iced_x86::*;
use anyhow::Result;

pub fn endbr64_handler(_instr: Instruction) -> Result<()> {
    Ok(())
}