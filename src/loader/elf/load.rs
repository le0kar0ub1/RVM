#![allow(dead_code)]

extern crate libc;
extern crate goblin;

use crate::arch;

use std::io::prelude::*;
use std::fs::File;

use anyhow::{Context, Result};

#[derive(Debug)]
pub struct ElfImg {
    file: String,
    img: *mut u8,
    buf: Vec<u8>,
}

pub enum ElfError {
    InvalidSymName,
    InvalidSymAddr,
    InvalidSec,
    FatalFileOp,
    FatalMemIO,
}

pub type ElfResult<T> = Result<T, ElfError>;

impl ElfImg {
    pub fn new(file: &String) -> Result<ElfImg> {
        let path = std::path::Path::new(file);
        let buf = std::fs::read(path).with_context(|| format!("Failed to read the given path: {:?}", path))?;
        let binobj = goblin::elf::Elf::parse(&buf).with_context(|| "Invalid ELF format")?;
        match binobj.header.e_machine {
            goblin::elf::header::EM_X86_64 => (),
            goblin::elf::header::EM_386 => (),
            goblin::elf::header::EM_AARCH64 => (),
            _ => panic!("Invalid target architecture")
        }
        let mut max: u64 = 0;
        for phdr in &binobj.program_headers {
            if max < phdr.p_vaddr + phdr.p_memsz {
                max = phdr.p_vaddr + phdr.p_memsz;
            }
        }
        max = (max + (1024 - 1)) & !(1024 - 1);
        let ptr = unsafe {
            let ptr: *mut u8 = libc::malloc(max as usize) as *mut u8;
            if ptr.is_null() {
                panic!("failed to allocate memory for processus image, require minimal size: {}", max);
            }
            ptr
        };
        println!("elf parser initialized");
        Ok (ElfImg {
            file: file.clone(),
            img: ptr,
            buf: buf,
        })
    }

    pub fn destroy(&mut self) {
        unsafe { 
            libc::free(self.img as *mut libc::c_void);
        }
    }

    fn read_whole_file(file: &String) -> std::io::Result<Vec<u8>> {
        let mut file = File::open(file)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        return Ok(data);
    }

    /* 
     * Write EHDR/PHDR/SHDR into image
    */
    fn load_static_hdrs(&mut self) -> Result<()> {
        let binobj = goblin::elf::Elf::parse(&self.buf)?;
        let wr = self.img as *mut u8;
        unsafe {
            for i in 0..(binobj.header.e_ehsize as usize) {
                std::ptr::write(wr.wrapping_add(i), self.buf[i]);
            }
        }
        // arch::x86::x86_64::load::load_phdrs(self.img, vec, 64, self.bin.sections.len());
        // wr = wr.wrapping_add(self.bin)
        Ok(())
    }

    pub fn load(&mut self) -> Result<()> {
        // let buffer = ElfImg::read_whole_file(&self.file)?;
        let binobj = goblin::elf::Elf::parse(&self.buf)?;
        self.load_static_hdrs()?;
        Ok(())
    }

    // pub fn SymAddrFromName(&self) {}
    // pub fn SymNameFromAddr(&self) {}
}