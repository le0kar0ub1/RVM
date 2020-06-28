#![allow(unused_variables)]
#![allow(dead_code)]

use crate::mem::comp;

use anyhow::{Result, anyhow};

pub struct Mem {
    stack: comp::stack::Stack,
    segments: Vec<comp::segments::Segment>,
    trans: usize,
    imglow: usize,
    imghigh: usize,
}

/*
 * Init with shitty values
 * here is stored only the static memory 
*/
static mut STATIC_MEM: Mem = Mem {
    stack: comp::stack::Stack {
                addr: 0,
                size: 0,
            },
    segments: vec!(),
    trans: 0,
    imglow: 0,
    imghigh: 0,
};

pub fn init(stacksz: u64, mut segments: Vec<comp::segments::Segment>, trans: usize, imglow: usize, imghigh: usize) -> Result<()> {
    let stack = comp::stack::Stack::new(stacksz)?;
    segments.push(
        comp::segments::Segment::new(
            stack.addr as usize, stack.size as usize, comp::segments::SegmentFlag::RW
        )
    );
    unsafe {
        STATIC_MEM.stack = stack;
        STATIC_MEM.segments = segments;
        STATIC_MEM.trans = trans;
        STATIC_MEM.imglow = imglow;
        STATIC_MEM.imghigh = imghigh;
    }
    Ok(())
}

pub fn stack_get() -> comp::stack::Stack {
    unsafe {
        STATIC_MEM.stack
    }
}

pub fn segment_add(addr: usize, size: usize, flags: comp::segments::SegmentFlag) {
    unsafe { 
        STATIC_MEM.segments.push(comp::segments::Segment::new(addr, size, flags));
    }
}

pub fn segment_get(addr: usize) -> Result<comp::segments::Segment> {
    unsafe {
        for seg in &STATIC_MEM.segments {
            if seg.addr <= addr && seg.addr + seg.size >= addr {
                return Ok(*seg)
            }
        }
    }
    Err(anyhow!(format!("Invalid segment requested {:#X}", addr)))
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
        for srch in 0..STATIC_MEM.segments.len() {
            if seg.addr == STATIC_MEM.segments[srch].addr {
                STATIC_MEM.segments.remove(srch);
                break;
            }
        }
    }
}

/*
 * Called before safe check
*/
fn check_mmap_supervis(addr: usize, min: comp::segments::SegmentFlag) -> Result<()> {
    let maps = procfs::process::Process::myself()?.maps()?;
    for map in maps {
        if addr > map.address.0 as usize && addr < map.address.1 as usize {
            let cur = match map.perms.as_str() {
                "r-xp" => comp::segments::SegmentFlag::RX,
                "rw-p" => comp::segments::SegmentFlag::RW,
                "r--p" => comp::segments::SegmentFlag::R,
                "rwxp" => comp::segments::SegmentFlag::RWX,
                 _     => comp::segments::SegmentFlag::R,
            };
            if min as u8 & cur as u8 != 0 {
                return Ok(())
            } else {
                break;
            }
        }
    }
    Err(anyhow!(format!("want {:?} address {} not mapped", min, addr)))
}

pub fn is_segment_valid(addr: usize, seg: comp::segments::SegmentFlag) -> Result<()> {
    check_mmap_supervis(addr, seg)?;
    Ok(())
}

pub fn is_segment_writable(addr: usize) -> Result<()> {
    is_segment_valid(addr, comp::segments::SegmentFlag::W)?;
    Ok(())
}

pub fn is_segment_executable(addr: usize) -> Result<()> {
    is_segment_valid(addr, comp::segments::SegmentFlag::X)?;
    Ok(())
}

pub fn is_segment_readable(addr: usize) -> Result<()> {
    is_segment_valid(addr, comp::segments::SegmentFlag::R)?;
    Ok(())
}

pub fn iftranslation(addr: usize) -> usize {
    unsafe {
        if addr >= STATIC_MEM.imglow && addr <= STATIC_MEM.imghigh {
            addr + STATIC_MEM.trans
        } else {
            addr
        }
    }
}