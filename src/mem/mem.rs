#![allow(unused_variables)]
#![allow(dead_code)]

use crate::mem::comp;

use anyhow::Result;

pub struct Mem {
    stack: comp::stack::Stack,
    segments: Vec<comp::segments::Segment>,
}

impl Mem {
    pub fn new(stacksz: u64) -> Result<Mem> {
        let stack = comp::stack::Stack::new(stacksz)?;
        Ok(Mem {
            stack: stack,
            segments: Vec::new()
        })
    }

    pub fn segment_add(&mut self, addr: usize, size: usize, flags: comp::segments::SegmentFlag) {
        self.segments.push(comp::segments::Segment::new(addr, size, flags));
    }

    pub fn segment_get(&self, addr: usize) -> comp::segments::SegmentFlag {
        for seg in &self.segments {
            if seg.addr <= addr && seg.addr + seg.size >= addr {
                return seg.flags
            }
        }
        comp::segments::SegmentFlag::Nop
    }
}