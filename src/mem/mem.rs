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
static mut mem: Mem = Mem {
    stack: comp::stack::Stack {
                addr: 0,
                size: 0,
            },
    segments: vec!(),
};

pub fn init(stacksz: u64, segments: Vec<comp::segments::Segment>) -> Result<()> {
    let stack = comp::stack::Stack::new(stacksz)?;
    unsafe {
        mem.stack = stack;
        mem.segments = segments;
    }
    Ok(())
}

pub fn stack_get() -> comp::stack::Stack {
    unsafe {
        mem.stack
    }
}

pub fn segment_add(addr: usize, size: usize, flags: comp::segments::SegmentFlag) {
    unsafe { 
        mem.segments.push(comp::segments::Segment::new(addr, size, flags));
    }
}

pub fn segment_get(addr: usize) -> Result<comp::segments::Segment> {
    unsafe {
        for seg in &mem.segments {
            if seg.addr <= addr && seg.addr + seg.size >= addr {
                return Ok(*seg)
            }
        }
    }
    Err(anyhow::anyhow!(format!("Invalid segment requested {}", addr)))
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
        for srch in 0..mem.segments.len() {
            if seg.addr == mem.segments[srch].addr {
                mem.segments.remove(srch);
                break;
            }
        }
    }
}
