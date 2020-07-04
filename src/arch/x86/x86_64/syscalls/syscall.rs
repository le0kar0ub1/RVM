#![allow(dead_code)]

use iced_x86::*;
use anyhow::Result;

use crate::arch::x86::shared::cpu;
use crate::arch::x86::x86_64::syscalls::systbl;

pub fn syscall_handler(_instr: Instruction) -> Result<()> {
    let idx = cpu::get64(Register::RAX)?;
    let ret = systbl::exec_syscall(idx as usize)?;
    cpu::set64(Register::RAX, ret as u64)?;
    Ok(())
}

pub fn obtain_arg(arg: usize) -> usize {
    match arg {
        1 => cpu::get64(Register::RDI).unwrap() as usize,
        2 => cpu::get64(Register::RSI).unwrap() as usize,
        3 => cpu::get64(Register::RDX).unwrap() as usize,
        4 => cpu::get64(Register::R10).unwrap() as usize,
        5 => cpu::get64(Register::R8).unwrap() as usize,
        6 => cpu::get64(Register::R9).unwrap() as usize,
        _ => 0,
    }
}

#[inline(always)]
pub unsafe fn syscall_exec0(n: usize) -> usize {
    let ret : usize;
    llvm_asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall_exec1(n: usize, a1: usize) -> usize {
    let ret : usize;
    llvm_asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall_exec2(n: usize, a1: usize, a2: usize) -> usize {
    let ret : usize;
    llvm_asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall_exec3(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let ret : usize;
    llvm_asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall_exec4(n: usize, a1: usize, a2: usize, a3: usize,
                                a4: usize) -> usize {
    let ret : usize;
    llvm_asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall_exec5(n: usize, a1: usize, a2: usize, a3: usize,
                                a4: usize, a5: usize) -> usize {
    let ret : usize;
    llvm_asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4), "{r8}"(a5)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall_exec6(n: usize, a1: usize, a2: usize, a3: usize,
                                a4: usize, a5: usize, a6: usize) -> usize {
    let ret : usize;
    llvm_asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4), "{r8}"(a5), "{r9}"(a6)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}