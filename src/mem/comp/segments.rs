#![allow(unused_must_use)]
#![allow(dead_code)]

pub enum SegmentFlag {
    Nop = 0x0,
    Img = 0x1,
    R   = 0x2,
    RW  = 0x3,
    RX  = 0x4,
    RWX = 0x5,
}

pub struct Segment {
    addr: usize,
    size: usize,
    flags: SegmentFlag,
}

impl Segment {
    pub fn new(addr: usize, size: usize, flags: SegmentFlag) -> Segment {
        Segment {
            addr,
            size,
            flags,
        }
    }
}