#![allow(dead_code)]

/*
 * A high level non-realist cheated ALU
 * Yeah, the goal is not to reproduce a computer logic
*/
use anyhow::{Result, anyhow};

use crate::arch::x86::shared::cpu;

fn opcheckup(xlen: u8, ylen: u8) -> Result<()> {
    if (xlen != 8 && xlen != 16 && xlen != 32 && xlen != 64) ||
    (ylen != 8 && ylen != 16 && ylen != 32 && ylen != 64) {
        Err(anyhow!("ALU critical error: operands lenght"))
    } else {
        Ok(())
    }
}

fn flagupdate(res: (u64, bool), len: u8) {
    let mut flg = cpu::supervis_get_flags_register();

    flg |= match res.1 {
        true => cpu::FlagRegister::OF as u64,
        false => 0 as u64,
    };
    flg |= match res.0 {
        0 => cpu::FlagRegister::ZF as u64,
        _ => 0,
    };
    flg |= match res.0 & (1 << (len - 1)) {
       0 => 0,
       _ => cpu::FlagRegister::SF as u64,
    };
    cpu::supervis_set_flags_register(flg);
}


pub fn add(x: u64, y: u64, xlen: u8, ylen: u8) -> Result<u64> {
    opcheckup(xlen, ylen)?;
    let mlen = if xlen > ylen { xlen } else { ylen };
    let res = x.overflowing_add(y);
    flagupdate(res, mlen);
    Ok(res.0)
}

pub fn sub(x: u64, y: u64, xlen: u8, ylen: u8) -> Result<u64> {
    opcheckup(xlen, ylen)?;
    let mlen = if xlen > ylen { xlen } else { ylen };
    let res = x.overflowing_sub(y);
    flagupdate(res, mlen);
    Ok(res.0)
}

pub fn or(x: u64, y: u64, xlen: u8, ylen: u8) -> Result<u64> {
    opcheckup(xlen, ylen)?;
    let mlen = if xlen > ylen { xlen } else { ylen };
    let res = x | y;
    flagupdate((res, false), mlen);
    Ok(res)
}

pub fn and(x: u64, y: u64, xlen: u8, ylen: u8) -> Result<u64> {
    opcheckup(xlen, ylen)?;
    let mlen = if xlen > ylen { xlen } else { ylen };
    let res = x & y;
    flagupdate((res, false), mlen);
    Ok(res)
}

pub fn xor(x: u64, y: u64, xlen: u8, ylen: u8) -> Result<u64> {
    opcheckup(xlen, ylen)?;
    let mlen = if xlen > ylen { xlen } else { ylen };
    let res = x ^ y;
    flagupdate((res, false), mlen);
    Ok(res)
}
