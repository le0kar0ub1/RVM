#![allow(dead_code)]

extern crate elf;
extern crate libc;

use std::path::PathBuf;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;

#[derive(Debug)]
pub struct ElfImg {
    file: String,
    img: *mut libc::c_void,
    bin: elf::File,
}

pub enum ElfError {
    InvalidSymName,
    InvalidSymAddr,
    InvalidSec,
    FataFileOp,
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
            let ptr: *mut libc::c_void = libc::malloc(max as usize) as *mut libc::c_void;
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

    pub fn exit(&mut self) {
        unsafe { 
            libc::free(self.img);
        }
    }

    fn read_whole_file(file: &String) -> std::io::Result<Vec<u8>> {
        let mut file = File::open(file)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        return Ok(data);
    }

    pub fn load(&mut self) -> ElfResult<()> {
        let mut buffer = match ElfImg::read_whole_file(&self.file) {
            Err(_err) => return Err(ElfError::FataFileOp),
            Ok(ok) => ok,
        };
        println!("{:?}", buffer[0]);
        Ok(())
    }

    // pub fn SymAddrFromName(&self) {}
    // pub fn SymNameFromAddr(&self) {}
}