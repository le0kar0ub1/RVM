#![allow(dead_code)]

extern crate libc;
extern crate goblin;

use crate::arch;

use std::io::prelude::*;
use std::fs::File;

use std::rc::Rc;

#[derive(Debug)]
pub struct ElfImg<'lt> {
    file: String,
    img: *mut u8,
    bin: goblin::elf::Elf<'lt>,
}

pub enum ElfError {
    InvalidSymName,
    InvalidSymAddr,
    InvalidSec,
    FatalFileOp,
    FatalMemIO,
}

const EHDR_SIZE: usize = 64;

pub type ElfResult<T> = Result<T, ElfError>;

impl <'lt>ElfImg<'lt> {
    pub fn new(file: &String) -> ElfImg<'lt> {
        let path = std::path::Path::new(file);
        let buf = std::fs::read(path).expect("Invalid given executable");
        let binobj = match goblin::elf::Elf::parse(&buf) {
            Ok(parsed) => parsed,
            Err(_e) => panic!("Elf parser failed")
        };
        println!("{:?}", binobj.program_headers);
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
        let ptr = unsafe {
            let ptr: *mut u8 = libc::malloc(max as usize) as *mut u8;
            if ptr.is_null() {
                panic!("failed to allocate memory for processus image, require minimal size: {}", max);
            }
            ptr
        };
        println!("elf parser initialized");
        ElfImg {
            file: file.clone(),
            img: ptr.clone(),
            bin: binobj,
        }
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
    fn load_static_hdrs(&mut self, vec: &Vec<u8>) {
        let wr = self.img as *mut u8;
        unsafe {
            for i in 0..EHDR_SIZE {
                std::ptr::write(wr.wrapping_add(i), vec[i]);
            }
        }
        // arch::x86::x86_64::load::load_phdrs(self.img, vec, 64, self.bin.sections.len())
        // wr = wr.wrapping_add(self.bin)
    }

    pub fn load(&mut self) -> ElfResult<()> {
        let buffer = match ElfImg::read_whole_file(&self.file) {
            Err(_err) => return Err(ElfError::FatalFileOp),
            Ok(ok) => ok,
        };
        self.load_static_hdrs(&buffer);
        Ok(())
    }

    // pub fn SymAddrFromName(&self) {}
    // pub fn SymNameFromAddr(&self) {}
}