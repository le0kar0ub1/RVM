#![allow(non_camel_case_types)]
#![allow(unused_must_use)]
#![allow(dead_code)]

/*
 * Some registers types
*/
pub type reg128 = u128;
pub type reg64  = u64;
pub type reg32  = u32;
pub type reg16  = u16;
pub type reg8   = u8;

extern crate libc;

use anyhow::Result;

use iced_x86::*;

/*
 * Struct describing the x64 processor
*/
pub struct Proc {
    rax: reg64,
    rbx: reg64,
    rcx: reg64,
    rdx: reg64,
    rsi: reg64,
    rdi: reg64,
    rbp: reg64,
    rsp: reg64,
    r8: reg64,
    r9: reg64,
    r10: reg64,
    r11: reg64,
    r12: reg64,
    r13: reg64,
    r14: reg64,
    r15: reg64,
    rip: reg64,
    rfl: reg64,
    cpl: reg64,
    es: reg16,
    cs: reg16,
    ss: reg16,
    ds: reg16,
    fs: reg16,
    gs: reg16,
    ldt: reg64,
    tr: reg64,
    gdt: reg64,
    idt: reg64,
    cr0: reg32,
    cr2: reg64,
    cr3: reg64,
    cr4: reg32,
    dr0: reg64,
    dr1: reg64,
    dr2: reg64,
    dr3: reg64,
    dr6: reg64,
    dr7: reg64,
    efer: reg64,
    fcw: reg64,
    fsw: reg64,
    ftw: reg64,
    mxcsr: reg64,
    fpr0: reg64,
    fpr1: reg64,
    fpr2: reg64,
    fpr3: reg64,
    fpr4: reg64,
    fpr5: reg64,
    fpr6: reg64,
    fpr7: reg64,
    xmm00: reg128,
    xmm01: reg128,
    xmm02: reg128,
    xmm03: reg128,
    xmm04: reg128,
    xmm05: reg128,
    xmm06: reg128,
    xmm07: reg128,
    xmm08: reg128,
    xmm09: reg128,
    xmm10: reg128,
    xmm11: reg128,
    xmm12: reg128,
    xmm13: reg128,
    xmm14: reg128,
    xmm15: reg128,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
enum RegisterSize {
    RegSz16  = 0b00,
    RegSz32  = 0b01,
    RegSz64  = 0b10,
    RegSz128 = 0b11,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
enum RegisterAccess {
    RegSys = 0b0,
    RegUsr = 0b1,
}

static mut CPU: Proc = Proc {
    rax : 0x0,
    rbx : 0x0,
    rcx : 0x0,
    rdx : 0x0,
    rsi : 0x0,
    rdi : 0x0,
    rbp : 0x0,
    rsp : 0x0,
    r8 : 0x0,
    r9 : 0x0,
    r10 : 0x0,
    r11 : 0x0,
    r12 : 0x0,
    r13 : 0x0,
    r14 : 0x0,
    r15 : 0x0,
    rip : 0x0,
    rfl : 0x0,
    cpl : 0x0,
    es : 0x0,
    cs : 0x0,
    ss : 0x0,
    ds : 0x0,
    fs : 0x0,
    gs : 0x0,
    ldt : 0x0,
    tr : 0x0,
    gdt : 0x0,
    idt : 0x0,
    cr0 : 0x0,
    cr2 : 0x0,
    cr3 : 0x0,
    cr4 : 0x0,
    dr0 : 0x0,
    dr1 : 0x0,
    dr2 : 0x0,
    dr3 : 0x0,
    dr6 : 0x0,
    dr7 : 0x0,
    efer : 0x0,
    fcw : 0x0,
    fsw : 0x0,
    ftw : 0x0,
    mxcsr : 0x0,
    fpr0 : 0x0,
    fpr1 : 0x0,
    fpr2 : 0x0,
    fpr3 : 0x0,
    fpr4 : 0x0,
    fpr5 : 0x0,
    fpr6 : 0x0,
    fpr7 : 0x0,
    xmm00 : 0x0,
    xmm01 : 0x0,
    xmm02 : 0x0,
    xmm03 : 0x0,
    xmm04 : 0x0,
    xmm05 : 0x0,
    xmm06 : 0x0,
    xmm07 : 0x0,
    xmm08 : 0x0,
    xmm09 : 0x0,
    xmm10 : 0x0,
    xmm11 : 0x0,
    xmm12 : 0x0,
    xmm13 : 0x0,
    xmm14 : 0x0,
    xmm15 : 0x0,
};

/*
 * Get register and returning his value
*/ 
pub fn get64_register(reg: Register) -> Result<reg64> {
    unsafe {
        match reg {
            Register::RAX => Ok(CPU.rax),
            Register::RBX => Ok(CPU.rbx),
            Register::RCX => Ok(CPU.rcx),
            Register::RDX => Ok(CPU.rdx),
            Register::RSI => Ok(CPU.rsi),
            Register::RDI => Ok(CPU.rdi),
            Register::RBP => Ok(CPU.rbp),
            Register::RSP => Ok(CPU.rsp),
            Register::R8 => Ok(CPU.r8),
            Register::R9 => Ok(CPU.r9),
            Register::R10 => Ok(CPU.r10),
            Register::R11 => Ok(CPU.r11),
            Register::R12 => Ok(CPU.r12),
            Register::R13 => Ok(CPU.r13),
            Register::R14 => Ok(CPU.r14),
            Register::R15 => Ok(CPU.r15),
            Register::RIP => Ok(CPU.rip),
            _ => Err(anyhow::anyhow!("Invalid register")),
        }
    }
}

pub fn get32_register(reg: Register) -> Result<reg32> {
    let res = get64_register(reg)?;
    Ok(res as reg32)
}

pub fn get16_register(reg: Register) -> Result<reg16> {
    let res = get64_register(reg)?;
    Ok(res as reg16)
}

pub fn get8_register(reg: Register) -> Result<reg8> {
    let res = get64_register(reg)?;
    Ok(res as reg8)
}

/*
 * Set the register with given value
*/
pub fn set64_register(reg: Register, val: reg64) -> Result<()> {
    unsafe {
        match reg {
            Register::RAX => { CPU.rax = val; Ok(()) },
            Register::RBX => { CPU.rbx = val; Ok(()) },
            Register::RCX => { CPU.rcx = val; Ok(()) },
            Register::RDX => { CPU.rdx = val; Ok(()) },
            Register::RSI => { CPU.rsi = val; Ok(()) },
            Register::RDI => { CPU.rdi = val; Ok(()) },
            Register::RBP => { CPU.rbp  = val; Ok(()) },
            Register::RSP => { CPU.rsp  = val; Ok(()) },
            Register::R8 => { CPU.r8 = val; Ok(()) },
            Register::R9 => { CPU.r9 = val; Ok(()) },
            Register::R10 => { CPU.r10 = val; Ok(()) },
            Register::R11 => { CPU.r11 = val; Ok(()) },
            Register::R12 => { CPU.r12 = val; Ok(()) },
            Register::R13 => { CPU.r13 = val; Ok(()) },
            Register::R14 => { CPU.r14 = val; Ok(()) },
            Register::R15 => { CPU.r15 = val; Ok(()) },
            Register::RIP => { CPU.rip = val; Ok(()) },
            _ => Err(anyhow::anyhow!("Invalid register")),
        }
    }
}

pub fn set32_register(reg: Register, val: reg32) -> Result<()> {
    let mut set: reg64 = get64_register(reg)?;
    set = (set & ((1 << 32) - 1)) | (val as reg64);
    set64_register(reg, set)?;
    Ok(())
}

pub fn set16_register(reg: Register, val: reg16) -> Result<()> {
    let mut set: reg64 = get64_register(reg)?;
    set = (set & ((1 << 16) - 1)) | (val as reg64);
    set64_register(reg, set)?;
    Ok(())
}

pub fn set8_register(reg: Register, val: reg8) -> Result<()> {
    let mut set: reg64 = get64_register(reg)?;
    set = (set & ((1 << 8) - 1)) | (val as reg64);
    set64_register(reg, set)?;
    Ok(())
}

pub fn init(stack: u64, ep: u64) {
    unsafe {
        CPU.rsp = stack;
        CPU.rbp = stack;
        CPU.rip = ep;
    }
}