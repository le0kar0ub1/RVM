#![allow(dead_code)]

use anyhow::Result;

use crate::mem::mem;

pub fn unsafe_get8(addr: usize) -> u8 {
    let hit = addr as *mut u8;
    unsafe {
        *hit
    }
}

pub fn unsafe_get16(addr: usize) -> u16 {
    let hit = addr as *mut u16;
    unsafe {
        *hit
    }
}

pub fn unsafe_get32(addr: usize) -> u32 {
    let hit = addr as *mut u32;
    unsafe {
        *hit
    }
}

pub fn unsafe_get64(addr: usize) -> u64 {
    let hit = addr as *mut u64;
    unsafe {
        *hit
    }
}

pub fn unsafe_set8(addr: usize, val: u8) {
    let hit = addr as *mut u8;
    unsafe {
        *hit = val; 
    }
}

pub fn unsafe_set16(addr: usize, val: u16) {
    let hit = addr as *mut u16;
    unsafe {
        *hit = val;
    }
}

pub fn unsafe_set32(addr: usize, val: u32) {
    let hit = addr as *mut u32;
    unsafe {
        *hit = val;
    }
}

pub fn unsafe_set64(addr: usize, val: u64) {
    let hit = addr as *mut u64;
    unsafe {
        *hit = val;
    }
}


pub fn safe_get8(addr: usize) -> Result<u8> {
    mem::is_segment_readable(addr)?;
    Ok(unsafe_get8(addr))
}

pub fn safe_get16(addr: usize) -> Result<u16> {
    mem::is_segment_readable(addr)?;
    Ok(unsafe_get16(addr))
}

pub fn safe_get32(addr: usize) -> Result<u32> {
    mem::is_segment_readable(addr)?;
    Ok(unsafe_get32(addr))
}

pub fn safe_get64(addr: usize) -> Result<u64> {
    mem::is_segment_readable(addr)?;
    Ok(unsafe_get64(addr))
}

pub fn safe_set8(addr: usize, val: u8) -> Result<()> {
    mem::is_segment_writable(addr)?;
    Ok(unsafe_set8(addr, val))
}

pub fn safe_set16(addr: usize, val: u16) -> Result<()> {
    mem::is_segment_writable(addr)?;
    Ok(unsafe_set16(addr, val))
}

pub fn safe_set32(addr: usize, val: u32) -> Result<()> {
    mem::is_segment_writable(addr)?;
    Ok(unsafe_set32(addr, val))
}

pub fn safe_set64(addr: usize, val: u64) -> Result<()> {
    mem::is_segment_writable(addr)?;
    Ok(unsafe_set64(addr, val))
}