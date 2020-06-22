#![allow(non_camel_case_types)]
#![allow(unused_must_use)]
#![allow(dead_code)]

extern crate libc;

use anyhow::Result;

use iced_x86::*;

/*
 * Struct describing the x64 processor
*/
pub struct Proc {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rbp: u64,
    rsp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rip: u64,
    rfl: u64,
    cpl: u64,
    es: u16,
    cs: u16,
    ss: u16,
    ds: u16,
    fs: u16,
    gs: u16,
    ldt: u64,
    tr: u64,
    gdt: u64,
    idt: u64,
    cr0: u32,
    cr2: u64,
    cr3: u64,
    cr4: u32,
    dr0: u64,
    dr1: u64,
    dr2: u64,
    dr3: u64,
    dr6: u64,
    dr7: u64,
    efer: u64,
    fcw: u64,
    fsw: u64,
    ftw: u64,
    mxcsr: u64,
    fpr0: u64,
    fpr1: u64,
    fpr2: u64,
    fpr3: u64,
    fpr4: u64,
    fpr5: u64,
    fpr6: u64,
    fpr7: u64,
    xmm00: u128,
    xmm01: u128,
    xmm02: u128,
    xmm03: u128,
    xmm04: u128,
    xmm05: u128,
    xmm06: u128,
    xmm07: u128,
    xmm08: u128,
    xmm09: u128,
    xmm10: u128,
    xmm11: u128,
    xmm12: u128,
    xmm13: u128,
    xmm14: u128,
    xmm15: u128,
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
pub fn get64_register(reg: Register) -> Result<u64> {
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

pub fn get32_register(reg: Register) -> Result<u32> {
    unsafe {
        match reg {
            Register::EAX => Ok(CPU.rax as u32),
            Register::EBX => Ok(CPU.rbx as u32),
            Register::ECX => Ok(CPU.rcx as u32),
            Register::EDX => Ok(CPU.rdx as u32),
            Register::ESI => Ok(CPU.rsi as u32),
            Register::EDI => Ok(CPU.rdi as u32),
            Register::EBP => Ok(CPU.rbp as u32),
            Register::ESP => Ok(CPU.rsp as u32),
            Register::R8 => Ok(CPU.r8 as u32),
            Register::R9D => Ok(CPU.r9 as u32),
            Register::R10D => Ok(CPU.r10 as u32),
            Register::R11D => Ok(CPU.r11 as u32),
            Register::R12D => Ok(CPU.r12 as u32),
            Register::R13D => Ok(CPU.r13 as u32),
            Register::R14D => Ok(CPU.r14 as u32),
            Register::R15D => Ok(CPU.r15 as u32),
            Register::EIP => Ok(CPU.rip as u32),
            _ => Err(anyhow::anyhow!("Invalid register")),
        }
    }
}

pub fn get16_register(reg: Register) -> Result<u16> {
    unsafe {
        match reg {
            Register::AX => Ok(CPU.rax as u16),
            Register::BX => Ok(CPU.rbx as u16),
            Register::CX => Ok(CPU.rcx as u16),
            Register::DX => Ok(CPU.rdx as u16),
            Register::SI => Ok(CPU.rsi as u16),
            Register::DI => Ok(CPU.rdi as u16),
            Register::BP => Ok(CPU.rbp as u16),
            Register::SP => Ok(CPU.rsp as u16),
            Register::R8W => Ok(CPU.r8 as u16),
            Register::R9W => Ok(CPU.r9 as u16),
            Register::R10W => Ok(CPU.r10 as u16),
            Register::R11W => Ok(CPU.r11 as u16),
            Register::R12W => Ok(CPU.r12 as u16),
            Register::R13W => Ok(CPU.r13 as u16),
            Register::R14W => Ok(CPU.r14 as u16),
            Register::R15W => Ok(CPU.r15 as u16),
            // Register::IP  => Ok(CPU.rip as u16),
            _ => Err(anyhow::anyhow!("Invalid register")),
        }
    }
}

pub fn get8_register(reg: Register) -> Result<u8> {
    unsafe {
        match reg {
            Register::AL => Ok(CPU.rax as u8),
            Register::BL => Ok(CPU.rbx as u8),
            Register::CL => Ok(CPU.rcx as u8),
            Register::DL => Ok(CPU.rdx as u8),
            Register::SIL => Ok(CPU.rsi as u8),
            Register::DIL => Ok(CPU.rdi as u8),
            Register::SPL => Ok(CPU.rsp as u8),
            Register::BPL => Ok(CPU.rbp as u8),

            Register::AH => Ok(((CPU.rax as u16) >> 8) as u8),
            Register::BH => Ok(((CPU.rbx as u16) >> 8) as u8),
            Register::CH => Ok(((CPU.rcx as u16) >> 8) as u8),
            Register::DH => Ok(((CPU.rdx as u16) >> 8) as u8),

            Register::R8L => Ok(CPU.r8 as u8),
            Register::R9L => Ok(CPU.r9 as u8),
            Register::R10L => Ok(CPU.r10 as u8),
            Register::R11L => Ok(CPU.r11 as u8),
            Register::R12L => Ok(CPU.r12 as u8),
            Register::R13L => Ok(CPU.r13 as u8),
            Register::R14L => Ok(CPU.r14 as u8),
            Register::R15L => Ok(CPU.r15 as u8),
            _ => Err(anyhow::anyhow!("Invalid register")),
        }
    }
}

/*
 * Set the register with given value
*/
pub fn set64_register(reg: Register, val: u64) -> Result<()> {
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

pub fn set32_register(reg: Register, val: u32) -> Result<()> {
    let reg = reg.full_register();
    let mut set: u64 = get64_register(reg)?;
    set = (set & ((1 << 32) - 1)) | (val as u64);
    set64_register(reg, set)?;
    Ok(())
}

pub fn set16_register(reg: Register, val: u16) -> Result<()> {
    let reg = reg.full_register();
    let mut set: u64 = get64_register(reg)?;
    set = (set & ((1 << 16) - 1)) | (val as u64);
    set64_register(reg, set)?;
    Ok(())
}

pub fn set8_register(reg: Register, val: u8) -> Result<()> {
    let reg = reg.full_register();
    let mut set: u64 = get64_register(reg)?;
    set = (set & ((1 << 8) - 1)) | (val as u64);
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

pub fn dump_reg(reg: Register) {
    let val = unsafe {
        match reg {
            Register::RAX => CPU.rax,
            Register::RBX => CPU.rbx,
            Register::RCX => CPU.rcx,
            Register::RDX => CPU.rdx,
            Register::RSI => CPU.rsi,
            Register::RDI => CPU.rdi,
            Register::RBP => CPU.rbp,
            Register::RSP => CPU.rsp,
            Register::R8 => CPU.r8,
            Register::R9 => CPU.r9,
            Register::R10 => CPU.r10,
            Register::R11 => CPU.r11,
            Register::R12 => CPU.r12,
            Register::R13 => CPU.r13,
            Register::R14 => CPU.r14,
            Register::R15 => CPU.r15,
            Register::RIP => CPU.rip,
            _ => 0,
        }
    };
    println!("{:?}: {}\n", reg, val);
}

pub fn dump_user_base_regs() {
    unsafe {
        println!("{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
                  \r{:?}: {:#X}
               ",
            Register::RAX, CPU.rax,
            Register::RBX, CPU.rbx,
            Register::RCX, CPU.rcx,
            Register::RDX, CPU.rdx,
            Register::RSI, CPU.rsi,
            Register::RDI, CPU.rdi,
            Register::RBP, CPU.rbp,
            Register::RSP, CPU.rsp,
            Register::R8,  CPU.r8,
            Register::R9,  CPU.r9,
            Register::R10, CPU.r10,
            Register::R11, CPU.r11,
            Register::R12, CPU.r12,
            Register::R13, CPU.r13,
            Register::R14, CPU.r14,
            Register::R15, CPU.r15,
            Register::RIP, CPU.rip
        )
    }
}