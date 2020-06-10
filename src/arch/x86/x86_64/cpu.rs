#![allow(non_camel_case_types)]
#![allow(unused_must_use)]
#![allow(dead_code)]

type reg64 = u64;
type reg32 = u32;
type reg16 = u16;
type reg8  = u8;

pub struct X64UserProc {
    rax: reg64,
    rdi: reg64,
    rsi: reg64,
    rdx: reg64,
    rbx: reg64,
    rcx: reg64,
    
    r8:  reg64,
    r9:  reg64,
    r10: reg64,
    r11: reg64,
    r12: reg64,
    r13: reg64,
    r14: reg64,
    r15: reg64,
    
    rbp: reg64,
    rsp: reg64,
    
    rip: reg64,
}

#[derive(Debug, Clone, Copy)]
pub enum UserRegister {
    rax,
    rdi,
    rsi,
    rdx,
    rbx,
    rcx,
    r8,
    r9,
    r10,
    r11,
    r12,
    r13,
    r14,
    r15,
    rbp,
    rsp,
    rip,
}

pub enum ProcError {
    InvalidRegister,
}

pub type ProcResult<T> = Result<T, ProcError>;

impl X64UserProc {
    pub fn new() -> X64UserProc {
        X64UserProc {
            rax: 0x0,
            rdi: 0x0,
            rsi: 0x0,
            rdx: 0x0,
            rbx: 0x0,
            rcx: 0x0,
            r8:  0x0,
            r9:  0x0,
            r10: 0x0,
            r11: 0x0,
            r12: 0x0,
            r13: 0x0,
            r14: 0x0,
            r15: 0x0,
            rbp: 0x0,
            rsp: 0x0,
            rip: 0x0,
        }
    }

    pub fn get64_register(&self, reg: UserRegister) -> ProcResult<reg64> {
        Ok (match reg {
            UserRegister::rax => self.rax,
            UserRegister::rdi => self.rdi,
            UserRegister::rsi => self.rsi,
            UserRegister::rdx => self.rdx,
            UserRegister::rbx => self.rbx,
            UserRegister::rcx => self.rcx,
            UserRegister::r8  => self.r8,
            UserRegister::r9  => self.r9,
            UserRegister::r10 => self.r10,
            UserRegister::r11 => self.r11,
            UserRegister::r12 => self.r12,
            UserRegister::r13 => self.r13,
            UserRegister::r14 => self.r14,
            UserRegister::r15 => self.r15,
            UserRegister::rbp => self.rbp,
            UserRegister::rsp => self.rsp,
            UserRegister::rip => self.rip,
        })
    }

    pub fn get32_register(&self, reg: UserRegister) -> ProcResult<reg32> {
        let res = self.get64_register(reg)?;
        Ok(res as reg32)
    }

    pub fn get16_register(&self, reg: UserRegister) -> ProcResult<reg16> {
        let res = self.get64_register(reg)?;
        Ok(res as reg16)
    }

    pub fn get8_register(&self, reg: UserRegister) -> ProcResult<reg8> {
        let res = self.get64_register(reg)?;
        Ok(res as reg8)
    }

    pub fn set64_register(&mut self, reg: UserRegister, val: reg64) -> ProcResult<()> {
        match reg {
            UserRegister::rax => self.rax = val,
            UserRegister::rdi => self.rdi = val,
            UserRegister::rsi => self.rsi = val,
            UserRegister::rdx => self.rdx = val,
            UserRegister::rbx => self.rbx = val,
            UserRegister::rcx => self.rcx = val,
            UserRegister::r8  => self.r8  = val,
            UserRegister::r9  => self.r9  = val,
            UserRegister::r10 => self.r10 = val,
            UserRegister::r11 => self.r11 = val,
            UserRegister::r12 => self.r12 = val,
            UserRegister::r13 => self.r13 = val,
            UserRegister::r14 => self.r14 = val,
            UserRegister::r15 => self.r15 = val,
            UserRegister::rbp => self.rbp = val,
            UserRegister::rsp => self.rsp = val,
            UserRegister::rip => self.rip = val,
        }
        Ok(())
    }

    pub fn set32_register(&mut self, reg: UserRegister, val: reg32) -> ProcResult<()> {
        let mut set: reg64 = self.get64_register(reg)?;
        set = (set & ((1 << 32) - 1)) | (val as reg64);
        self.set64_register(reg, set)?;
        Ok(())
    }

    pub fn set16_register(&mut self, reg: UserRegister, val: reg16) -> ProcResult<()> {
        let mut set: reg64 = self.get64_register(reg)?;
        set = (set & ((1 << 16) - 1)) | (val as reg64);
        self.set64_register(reg, set)?;
        Ok(())
    }

    pub fn set8_register(&mut self, reg: UserRegister, val: reg8) -> ProcResult<()> {
        let mut set: reg64 = self.get64_register(reg)?;
        set = (set & ((1 << 8) - 1)) | (val as reg64);
        self.set64_register(reg, set)?;
        Ok(())
    }
}

// static mut stack: [u8; 0x100000] = [0; 0x100000];

pub fn init() {

}