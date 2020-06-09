#[macro_use]
use lazy_static::lazy_static;

#[allow(non_camel_case_types)]
struct x64_abiprocessor {
    rax: u64,
    rdi: u64,
    rsi: u64,
    rdx: u64,
    rbx: u64,
    rcx: u64,

    r8:  u64,
    r9:  u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,

    rbp: u64,
    rsp: u64,

    rip: u64,

    rflags: u64,

    ds:  u16,
    es:  u16,
    ss:  u16,
    fs:  u16,
    gs:  u16,
    cs:  u16,
}

// type stack_t = [u8; 0x100000];

// lazy_static! {
//     static ref GLOBAL: Arc<Mutex<GlobalType> =
//         Arc::new(Mutex::new(GlobalType::new()));
// }

pub fn init() {
}