#![allow(unused_must_use)]
#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SegmentFlag {
    Nop = 0x0,
    R   = 0x4,
    RW  = 0x6,
    RX  = 0x5,
    RWX = 0x7,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SegmentError {
    SegmentNotFound,
    SegmentDirty,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment {
    pub addr: usize,
    pub size: usize,
    pub flags: SegmentFlag,
    pub dirty: bool,
}

impl Segment {
    pub fn new(addr: usize, size: usize, flags: SegmentFlag) -> Segment {
        Segment {
            addr,
            size,
            flags,
            dirty: false,
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn soiled(&mut self) {
        self.dirty = true;
    }

    pub fn unsoiled(&mut self) {
        self.dirty = false;
    }
}