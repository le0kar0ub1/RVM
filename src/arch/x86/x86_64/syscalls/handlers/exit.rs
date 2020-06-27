use crate::arch::x86::x86_64::scheduler;
use crate::arch::x86::x86_64::syscalls::syscall;

use anyhow::Result;

pub fn syscall_exit(_slf: usize) -> Result<usize> {
    let exitcode = syscall::obtain_arg(1);
    scheduler::arch_exit(exitcode as i32);
    Ok(0)
}