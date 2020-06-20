#![allow(unused_must_use)]
#![allow(dead_code)]

extern crate libc;

use anyhow::Result;

#[derive(Copy, Clone, Debug)]
pub struct Stack {
    pub addr: usize,
    pub size: u64,
}

impl Stack {
    pub fn new(size: u64) -> Result<Stack> {
        let ptr = unsafe {
            let ptr: *mut u8 = libc::calloc(size as usize, 1) as *mut u8;
            if ptr.is_null() {
                return Err(anyhow::anyhow!(format!("failed to allocate stack for processus, require minimal size: {}", size)));
            }
            ptr as usize
        };
        Ok(Stack {
            addr: ptr,
            size: size,
        })
    }

    pub fn destroy(&mut self) {
        unsafe { 
            libc::free(self.addr as *mut libc::c_void);
        }
    }

    pub fn get_addr(&self) -> usize {
        self.addr
    }
}