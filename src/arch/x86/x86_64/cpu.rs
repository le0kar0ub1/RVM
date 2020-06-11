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

/*
 * Struct describing a minimal x64 processor for userspace utilization
*/
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

/*
 * x64 Register list
*/
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
    xmm00,
    xmm01,
    xmm02,
    xmm03,
    xmm04,
    xmm05,
    xmm06,
    xmm07,
    xmm08,
    xmm09,
    xmm10,
    xmm11,
    xmm12,
    xmm13,
    xmm14,
    xmm15,
}

pub enum KernelRegisters {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
    RSP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    RIP,
    RFL,
    CPL,
    ES,
    CS,
    SS,
    DS,
    FS,
    GS,
    LDT,
    TR,
    GDT, 
    IDT,
    CR0,
    CR2,
    CR3,
    CR4,
    DR0,
    DR1,
    DR2,
    DR3,
    DR6,
    DR7,
    EFER,
    FCW,
    FSW,
    FTW,
    MXCSR,
    FPR0,
    FPR1,
    FPR2,
    FPR3,
    FPR4,
    FPR5,
    FPR6,
    FPR7,
    XMM00,
    XMM01,
    XMM02,
    XMM03,
    XMM04,
    XMM05,
    XMM06,
    XMM07,
    XMM08,
    XMM09,
    XMM10,
    XMM11,
    XMM12,
    XMM13,
    XMM14,
    XMM15,
}


/*
 * Unused Error handling at time 
*/
pub enum ProcError {
    InvalidRegister,
}

pub type ProcResult<T> = Result<T, ProcError>;

impl X64UserProc {
    /*
     * Create a x64 processor
    */
    pub fn new() -> X64UserProc {
        X64UserProc {
            rax:   0x0,
            rdi:   0x0,
            rsi:   0x0,
            rdx:   0x0,
            rbx:   0x0,
            rcx:   0x0,
            r8:    0x0,
            r9:    0x0,
            r10:   0x0,
            r11:   0x0,
            r12:   0x0,
            r13:   0x0,
            r14:   0x0,
            r15:   0x0,
            rbp:   0x0,
            rsp:   0x0,
            rip:   0x0,
            xmm00: 0x0,
            xmm01: 0x0,
            xmm02: 0x0,
            xmm03: 0x0,
            xmm04: 0x0,
            xmm05: 0x0,
            xmm06: 0x0,
            xmm07: 0x0,
            xmm08: 0x0,
            xmm09: 0x0,
            xmm10: 0x0,
            xmm11: 0x0,
            xmm12: 0x0,
            xmm13: 0x0,
            xmm14: 0x0,
            xmm15: 0x0,
        }
    }

    /*
     * Get register and returning his value
    */ 
    pub fn get64_register(&self, reg: UserRegister) -> ProcResult<reg64> {
        match reg {
            UserRegister::rax => Ok(self.rax),
            UserRegister::rdi => Ok(self.rdi),
            UserRegister::rsi => Ok(self.rsi),
            UserRegister::rdx => Ok(self.rdx),
            UserRegister::rbx => Ok(self.rbx),
            UserRegister::rcx => Ok(self.rcx),
            UserRegister::r8  => Ok(self.r8),
            UserRegister::r9  => Ok(self.r9),
            UserRegister::r10 => Ok(self.r10),
            UserRegister::r11 => Ok(self.r11),
            UserRegister::r12 => Ok(self.r12),
            UserRegister::r13 => Ok(self.r13),
            UserRegister::r14 => Ok(self.r14),
            UserRegister::r15 => Ok(self.r15),
            UserRegister::rbp => Ok(self.rbp),
            UserRegister::rsp => Ok(self.rsp),
            UserRegister::rip => Ok(self.rip),
            _ => Err(ProcError::InvalidRegister),
        }
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

    /*
     * Set the register with given value
    */
    pub fn set64_register(&mut self, reg: UserRegister, val: reg64) -> ProcResult<()> {
        match reg {
            UserRegister::rax => { self.rax = val; Ok(()) },
            UserRegister::rdi => { self.rdi = val; Ok(()) },
            UserRegister::rsi => { self.rsi = val; Ok(()) },
            UserRegister::rdx => { self.rdx = val; Ok(()) },
            UserRegister::rbx => { self.rbx = val; Ok(()) },
            UserRegister::rcx => { self.rcx = val; Ok(()) },
            UserRegister::r8  => { self.r8  = val; Ok(()) },
            UserRegister::r9  => { self.r9  = val; Ok(()) },
            UserRegister::r10 => { self.r10 = val; Ok(()) },
            UserRegister::r11 => { self.r11 = val; Ok(()) },
            UserRegister::r12 => { self.r12 = val; Ok(()) },
            UserRegister::r13 => { self.r13 = val; Ok(()) },
            UserRegister::r14 => { self.r14 = val; Ok(()) },
            UserRegister::r15 => { self.r15 = val; Ok(()) },
            UserRegister::rbp => { self.rbp = val; Ok(()) },
            UserRegister::rsp => { self.rsp = val; Ok(()) },
            UserRegister::rip => { self.rip = val; Ok(()) },
            _ => Err(ProcError::InvalidRegister),
        }
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

/*
 * supervisor init processor
*/
pub fn init(stack: reg64, entry: reg64) -> X64UserProc {
    let mut new = X64UserProc::new();
    new.rbp = stack;
    new.rsp = stack;
    new.rip = entry;
    println!("x64 Processor initialized");
    new
}