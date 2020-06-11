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
 * Struct describing a minimal  processor for userspace utilization
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
    RegSz8   = 0b000,
    RegSz16  = 0b001,
    RegSz32  = 0b010,
    RegSz64  = 0b011,
    RegSz128 = 0b100,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
enum RegisterAccess {
    RegSys  = 0b0,
    RegUser = 0b1,
}

/*
 *  Register list
*/
#[derive(Debug, Clone, Copy)]
pub enum Register {
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

impl Proc {
    /*
     * Create a  processor
    */
    pub fn new() -> Proc {
        Proc {
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
        }
    }

    /*
     * Get register and returning his value
    */ 
    pub fn get64_register(&self, reg: Register) -> ProcResult<reg64> {
        match reg {
            Register::RAX => Ok(self.rax),
            Register::RBX => Ok(self.rbx),
            Register::RCX => Ok(self.rcx),
            Register::RDX => Ok(self.rdx),
            Register::RSI => Ok(self.rsi),
            Register::RDI => Ok(self.rdi),
            Register::RBP => Ok(self.rbp),
            Register::RSP => Ok(self.rsp),
            Register::R8 => Ok(self.r8),
            Register::R9 => Ok(self.r9),
            Register::R10 => Ok(self.r10),
            Register::R11 => Ok(self.r11),
            Register::R12 => Ok(self.r12),
            Register::R13 => Ok(self.r13),
            Register::R14 => Ok(self.r14),
            Register::R15 => Ok(self.r15),
            Register::RIP => Ok(self.rip),
            _ => Err(ProcError::InvalidRegister),
        }
    }

    pub fn get32_register(&self, reg: Register) -> ProcResult<reg32> {
        let res = self.get64_register(reg)?;
        Ok(res as reg32)
    }

    pub fn get16_register(&self, reg: Register) -> ProcResult<reg16> {
        let res = self.get64_register(reg)?;
        Ok(res as reg16)
    }

    pub fn get8_register(&self, reg: Register) -> ProcResult<reg8> {
        let res = self.get64_register(reg)?;
        Ok(res as reg8)
    }

    /*
     * Set the register with given value
    */
    pub fn set64_register(&mut self, reg: Register, val: reg64) -> ProcResult<()> {
        match reg {
            Register::RAX => { self.rax = val; Ok(()) },
            Register::RBX => { self.rbx = val; Ok(()) },
            Register::RCX => { self.rcx = val; Ok(()) },
            Register::RDX => { self.rdx = val; Ok(()) },
            Register::RSI => { self.rsi = val; Ok(()) },
            Register::RDI => { self.rdi = val; Ok(()) },
            Register::RBP => { self.rbp  = val; Ok(()) },
            Register::RSP => { self.rsp  = val; Ok(()) },
            Register::R8 => { self.r8 = val; Ok(()) },
            Register::R9 => { self.r9 = val; Ok(()) },
            Register::R10 => { self.r10 = val; Ok(()) },
            Register::R11 => { self.r11 = val; Ok(()) },
            Register::R12 => { self.r12 = val; Ok(()) },
            Register::R13 => { self.r13 = val; Ok(()) },
            Register::R14 => { self.r14 = val; Ok(()) },
            Register::R15 => { self.r15 = val; Ok(()) },
            Register::RIP => { self.rip = val; Ok(()) },
            _ => Err(ProcError::InvalidRegister),
        }
    }

    pub fn set32_register(&mut self, reg: Register, val: reg32) -> ProcResult<()> {
        let mut set: reg64 = self.get64_register(reg)?;
        set = (set & ((1 << 32) - 1)) | (val as reg64);
        self.set64_register(reg, set)?;
        Ok(())
    }

    pub fn set16_register(&mut self, reg: Register, val: reg16) -> ProcResult<()> {
        let mut set: reg64 = self.get64_register(reg)?;
        set = (set & ((1 << 16) - 1)) | (val as reg64);
        self.set64_register(reg, set)?;
        Ok(())
    }

    pub fn set8_register(&mut self, reg: Register, val: reg8) -> ProcResult<()> {
        let mut set: reg64 = self.get64_register(reg)?;
        set = (set & ((1 << 8) - 1)) | (val as reg64);
        self.set64_register(reg, set)?;
        Ok(())
    }
}

/*
 * supervisor init processor
*/
pub fn init(stack: reg64, entry: reg64) -> Proc {
    let mut new = Proc::new();
    new.rbp = stack;
    new.rsp = stack;
    new.rip = entry;
    println!(" Processor initialized");
    new
}