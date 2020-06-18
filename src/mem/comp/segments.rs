#![allow(unused_must_use)]
#![allow(dead_code)]

use anyhow::Result;

#[derive(Clone, Copy, Debug)]
pub enum SegmentFlag {
    Nop = 0x0,
    Img = 0x1,
    R   = 0x2,
    RW  = 0x3,
    RX  = 0x4,
    RWX = 0x5,
}

#[derive(Clone, Copy, Debug)]
pub struct Segment {
    pub addr: usize,
    pub size: usize,
    pub flags: SegmentFlag,
}

impl Segment {
    pub fn new(addr: usize, size: usize, flags: SegmentFlag) -> Segment {
        Segment {
            addr,
            size,
            flags,
        }
    }

    pub fn destroy(&mut self, map: &mut Vec<Segment>) -> Result<()> {
        for seg in 0..map.len() {
            if self.addr == map[seg].addr {
                map.remove(seg);
            }
        }
        Ok(())
    }
}