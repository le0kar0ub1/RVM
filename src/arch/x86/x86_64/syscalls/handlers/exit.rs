use crate::arch::x86::x86_64::scheduler;
use crate::arch::x86::x86_64::syscalls::syscall;

pub fn syscall_exit(_slf: usize) -> usize {
    let exitcode = syscall::obtain_arg(1);
    scheduler::arch_exit(exitcode as i32);
    0
}