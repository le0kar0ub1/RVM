#![allow(unused_must_use)]
#![allow(dead_code)]

use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SegmentFlag {
    Nop = 0x0,
    Img = 0x1,
    R   = 0x2,
    RW  = 0x3,
    RX  = 0x4,
    RWX = 0x5,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SegmentError {
    SegmentNotFound = 0x0,
    SegmentDirty    = 0x1,
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