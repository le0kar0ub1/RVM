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
}