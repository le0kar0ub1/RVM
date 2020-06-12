#![allow(dead_code)]

extern crate elf;
extern crate libc;

use std::path::PathBuf;

use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
pub struct ElfImg {
    file: String,
    img: *mut u8,
    bin: elf::File,
}

pub enum ElfError {
    InvalidSymName,
    InvalidSymAddr,
    InvalidSec,
    FataFileOp,
    FataMeMIO,
}

pub type ElfResult<T> = Result<T, ElfError>;

impl ElfImg {
    pub fn new(file: &String) -> ElfImg {
        let path = PathBuf::from(file);
        let binobj = elf::File::open_path(&path).expect("Invalid given executable");
        match binobj.ehdr.machine {
            elf::types::EM_X86_64 => (),
            // elf::types::EM_386 => (),
            // elf::types::EM_AARCH64 => (),
            _ => panic!("Invalid target architecture")
        }

        let mut max: u64 = 0;
        for phdr in &binobj.phdrs {
            if max < phdr.vaddr + phdr.memsz {
                max = phdr.vaddr + phdr.memsz;
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
            img: ptr,
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
            for i in 0..64 {
                std::ptr::write(wr.wrapping_add(i), vec[i]);
            }
        }
        println!("phdrs: {}", self.bin.sections.len());
        // wr = wr.wrapping_add(self.bin)
    }

    pub fn load(&mut self) -> ElfResult<()> {
        let buffer = match ElfImg::read_whole_file(&self.file) {
            Err(_err) => return Err(ElfError::FataFileOp),
            Ok(ok) => ok,
        };
        self.load_static_hdrs(&buffer);
        Ok(())
    }

    // pub fn SymAddrFromName(&self) {}
    // pub fn SymNameFromAddr(&self) {}
}