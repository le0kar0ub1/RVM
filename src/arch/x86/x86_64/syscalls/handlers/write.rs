use crate::arch::x86::x86_64::syscalls::syscall;
use crate::mem::mem;

use anyhow::Result;

pub fn syscall_write(slf: usize) -> Result<usize> {
    let a1 = syscall::obtain_arg(1);
    let a3 = syscall::obtain_arg(3);
    let a2 = mem::is_segment_readable(syscall::obtain_arg(2) as usize, a3 as usize)?;
    unsafe {
        Ok(syscall::syscall_exec3(slf, a1, a2, a3))
    }
}