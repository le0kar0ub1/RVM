#![allow(unused_variables)]
#![allow(dead_code)]

use crate::mem::comp;

use anyhow::Result;

pub struct Mem {
    pub stack: comp::stack::Stack,
    segments: Vec<comp::segments::Segment>,
}

impl Mem {
    pub fn new(stacksz: u64, segments: Vec<comp::segments::Segment>) -> Result<Mem> {
        let stack = comp::stack::Stack::new(stacksz)?;
        Ok(Mem {
            stack: stack,
            segments: segments,
        })
    }

    pub fn segment_add(&mut self, addr: usize, size: usize, flags: comp::segments::SegmentFlag) {
        self.segments.push(comp::segments::Segment::new(addr, size, flags));
    }

    pub fn segment_get(&self, addr: usize) -> Result<&comp::segments::Segment> {
        for seg in &self.segments {
            if seg.addr <= addr && seg.addr + seg.size >= addr {
                return Ok(seg)
            }
        }
        Err(anyhow::anyhow!(format!("Invalid segment requested {}", addr)))
    }

    pub fn segment_get_flags(&self, addr: usize) -> comp::segments::SegmentFlag {
        let res = self.segment_get(addr);
        match res.is_ok() {
            true => res.unwrap().flags,
            false => comp::segments::SegmentFlag::Nop,
        }
    }

    pub fn segment_remove(&mut self, seg: &comp::segments::Segment) {
        for srch in 0..self.segments.len() {
            if seg.addr == self.segments[srch].addr {
                self.segments.remove(srch);
                break;
            }
        }
    }
}