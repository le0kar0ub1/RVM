#![allow(unused_variables)]
#![allow(dead_code)]

use crate::mem::comp;

use anyhow::Result;

pub struct Mem {
    stack: comp::stack::Stack,
    segments: Vec<comp::segments::Segment>,
}

/*
 * Init with shitty values
*/
static mut MEM: Mem = Mem {
    stack: comp::stack::Stack {
                addr: 0,
                size: 0,
            },
    segments: vec!(),
};

pub fn init(stacksz: u64, segments: Vec<comp::segments::Segment>) -> Result<()> {
    let stack = comp::stack::Stack::new(stacksz)?;
    unsafe {
        MEM.stack = stack;
        MEM.segments = segments;
    }
    Ok(())
}

pub fn stack_get() -> comp::stack::Stack {
    unsafe {
        MEM.stack
    }
}

pub fn segment_add(addr: usize, size: usize, flags: comp::segments::SegmentFlag) {
    unsafe { 
        MEM.segments.push(comp::segments::Segment::new(addr, size, flags));
    }
}

pub fn segment_get(addr: usize) -> Result<comp::segments::Segment> {
    unsafe {
        for seg in &MEM.segments {
            if seg.addr <= addr && seg.addr + seg.size >= addr {
                return Ok(*seg)
            }
        }
    }
    Err(anyhow::anyhow!(format!("Invalid segment requested {:#X}", addr)))
}

pub fn segment_get_flags(addr: usize) -> comp::segments::SegmentFlag {
    let res = segment_get(addr);
    match res.is_ok() {
        true => res.unwrap().flags,
        false => comp::segments::SegmentFlag::Nop,
    }
}

pub fn segment_remove(seg: &comp::segments::Segment) {
    unsafe {
        for srch in 0..MEM.segments.len() {
            if seg.addr == MEM.segments[srch].addr {
                MEM.segments.remove(srch);
                break;
            }
        }
    }
}

pub fn is_segment_valid(addr: usize) -> Result<()> {
    Ok(())
}

pub fn is_segment_writable(addr: usize) -> Result<()> {
    is_segment_valid(addr)?;
    Ok(())
}

pub fn is_segment_executable(addr: usize) -> Result<()> {
    is_segment_valid(addr)?;
    Ok(())
}

pub fn is_segment_readable(addr: usize) -> Result<()> {
    is_segment_valid(addr)?;
    Ok(())
}