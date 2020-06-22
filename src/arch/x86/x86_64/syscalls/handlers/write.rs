use crate::arch::x86::x86_64::syscalls::syscall;
use crate::mem::mem;

pub fn syscall_write(slf: usize) -> usize {
    let a1 = syscall::obtain_arg(1);
    let a2 = mem::iftranslation(syscall::obtain_arg(2));
    let a3 = syscall::obtain_arg(3);
    unsafe {
        syscall::syscall_exec3(slf, a1, a2, a3)
    }
}