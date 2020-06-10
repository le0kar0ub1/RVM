#![allow(non_camel_case_types)]
#![derive(Debug)]

type reg64 = u64;
type reg32 = u32;
type reg16 = u16;
type reg8  = u8;

struct X64UserProc {
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
    fn get64_register(self, reg: UserRegister) -> ProcResult<reg64> {
        match reg {
            UserRegister::rax => Ok(self.rax),
            UserRegister::rdi => Ok(self.rdi),
            UserRegister::rsi => Ok(self.rsi),
            UserRegister::rdx => Ok(self.rdx),
            UserRegister::rbx => Ok(self.rbx),
            UserRegister::rcx => Ok(self.rcx),
            UserRegister::r8 => Ok(self.r8),
            UserRegister::r9 => Ok(self.r9),
            UserRegister::r10 => Ok(self.r10),
            UserRegister::r11 => Ok(self.r11),
            UserRegister::r12 => Ok(self.r12),
            UserRegister::r13 => Ok(self.r13),
            UserRegister::r14 => Ok(self.r14),
            UserRegister::r15 => Ok(self.r15),
            UserRegister::rbp => Ok(self.rbp),
            UserRegister::rsp => Ok(self.rsp),
            UserRegister::rip => Ok(self.rip),
        }
    }

    fn get32_register(self, reg: UserRegister) {
        let res: reg64 = X64UserProc::get64_register(self, reg).ok().unwrap();
    }

    fn set64_register<T>(mut self, reg: UserRegister, val: reg64) -> ProcResult<()> {
        match reg {
            UserRegister::rax => self.rax = val,
            UserRegister::rdi => self.rdi = val,
            UserRegister::rsi => self.rsi = val,
            UserRegister::rdx => self.rdx = val,
            UserRegister::rbx => self.rbx = val,
            UserRegister::rcx => self.rcx = val,
            UserRegister::r8 => self.r8 = val,
            UserRegister::r9 => self.r9 = val,
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
}

// static mut stack: [u8; 0x100000] = [0; 0x100000];

// static mut x64processor: X64UserProc = X64UserProc {
//     rax: 0,
//     rdi: 0,
//     rsi: 0,
//     rdx: 0,
//     rbx: 0,
//     rcx: 0,
//     r8: 0,
//     r9: 0,
//     r10: 0,
//     r11: 0,
//     r12: 0,
//     r13: 0,
//     r14: 0,
//     r15: 0,
//     rbp: 0,
//     rsp: stack,
//     rip: stack,
//     rflags: 0,
//     ds: 0,
//     es: 0,
//     ss: 0,
//     fs: 0,
//     gs: 0,
//     cs: 0,
// };

pub fn init() {

}