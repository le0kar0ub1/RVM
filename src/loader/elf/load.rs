#![allow(dead_code)]

extern crate libc;
extern crate goblin;

// use crate::arch;

use std::io::prelude::*;
use std::fs::File;

use anyhow::{Context, Result};

#[derive(Debug)]
pub struct ElfImg {
    file: String,
    img: *mut u8,
    imgsz: usize,
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
        let mut max: usize = 0;
        for phdr in &binobj.program_headers {
            if max < (phdr.p_vaddr + phdr.p_memsz) as usize{
                max = (phdr.p_vaddr + phdr.p_memsz) as usize;
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
            imgsz: max,
            buf: buf,
        })
    }

    pub fn destroy(&mut self) {
        unsafe { 
            libc::free(self.img as *mut libc::c_void);
        }
    }

    #[allow(non_snake_case)]
    fn ToReallocOrNoToRealloc(&mut self, new: usize) {
        if new > self.imgsz {
            let ptr = unsafe {
                let ptr: *mut u8 = libc::realloc(self.img as *mut libc::c_void, new) as *mut u8;
                if ptr.is_null() {
                    panic!("failed to allocate memory for processus image, require minimal size: {}", new);
                }
                ptr
            };
            self.img = ptr;
            self.imgsz = new;
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
        let bufcp = &self.buf.clone();
        let binobj = goblin::elf::Elf::parse(bufcp)?;
        let ehdr = self.img as *mut u8;
        unsafe {
            for i in 0..(binobj.header.e_ehsize as usize) {
                std::ptr::write(ehdr.wrapping_add(i), self.buf[i]);
            }
        }
        self.ToReallocOrNoToRealloc((binobj.header.e_phoff as usize) + (binobj.header.e_phentsize * binobj.header.e_phnum) as usize);
        let phdr = self.img.wrapping_add(binobj.header.e_phoff as usize);
        unsafe {
            for i in 0..((binobj.header.e_phentsize * binobj.header.e_phnum) as usize) {
                std::ptr::write(phdr.wrapping_add(i), self.buf[i]);
            }
        }
        self.ToReallocOrNoToRealloc((binobj.header.e_shoff as usize) + (binobj.header.e_shentsize * binobj.header.e_shnum) as usize);
        let shdr = self.img.wrapping_add(binobj.header.e_shoff as usize);
        unsafe {
            for i in 0..((binobj.header.e_shentsize * binobj.header.e_shnum) as usize) {
                std::ptr::write(shdr.wrapping_add(i), self.buf[i]);
            }
        }
        // wr = wr.wrapping_add(self.bin)
        Ok(())
    }

    pub fn load(&mut self) -> Result<()> {
        // let buffer = ElfImg::read_whole_file(&self.file)?;
        // let binobj = goblin::elf::Elf::parse(&self.buf)?;
        self.load_static_hdrs()?;
        Ok(())
    }

    // pub fn SymAddrFromName(&self) {}
    // pub fn SymNameFromAddr(&self) {}
}