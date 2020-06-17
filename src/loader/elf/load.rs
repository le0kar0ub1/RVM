#![allow(dead_code)]

/*
 * A lot of unsafe block in this code part.
 * The problem is that we can't use rust collections to achieve. The data must be fully raw.
 * Bad usage of the language.
*/

extern crate libc;
extern crate goblin;

// use crate::arch;

use std::io::prelude::*;
use std::fs::File;

use anyhow::{Context, Result};

const DYNAMIC_LIBRARY_PATH_LINUX: &str = "/usr/lib/";

#[derive(Debug)]
pub struct ElfImg {
    file: String,
    img: *mut u8,
    imgsz: usize,
    buf: Vec<u8>,
}

#[derive(Debug)]
pub enum ElfError {
    InvalidSymName,
    InvalidSymAddr,
    InvalidSec,
    FatalFileOp,
    FatalMemIO,
}


#[allow(unused_macros)]
macro_rules! log {
    ($expression:expr) => {
        println!("[LOADER]: {}.", $expression);
    };
}

pub type ElfResult<T> = Result<T, ElfError>;

impl ElfImg {
    /*
     * Create a new instance ELF image
    */
    pub fn new(file: &String) -> Result<ElfImg> {
        let path = std::path::Path::new(file);
        let buf = std::fs::read(path).with_context(|| format!("Failed to read the given path: {:?}", path))?;
        let binobj = goblin::elf::Elf::parse(&buf).with_context(|| format!("Invalid ELF format: {:?}", path))?;
        match binobj.header.e_machine {
            goblin::elf::header::EM_X86_64 => (),
            goblin::elf::header::EM_386 => (),
            goblin::elf::header::EM_AARCH64 => (),
            _ => return Err(anyhow::anyhow!("Invalid target architecture")),
        }
        /*
         * Juste init with a first alloc
        */
        let ptr = unsafe {
            let ptr: *mut u8 = libc::calloc(0x100, 1) as *mut u8;
            if ptr.is_null() {
                return Err(anyhow::anyhow!(format!("failed to allocate memory for processus image, require minimal size: {}", 0x100)));
            }
            ptr
        };
        log!("parser initialized");
        Ok (ElfImg {
            file: file.clone(),
            img: ptr,
            imgsz: 0x40,
            buf: buf,
        })
    }

    /*
     * Free our process image when terminated
    */
    pub fn destroy(&mut self) {
        unsafe { 
            libc::free(self.img as *mut libc::c_void);
        }
    }

    /*
     * Take the new writing space and realloc if necessary
    */
    #[allow(non_snake_case)]
    fn ToReallocOrNoToRealloc(&mut self, new: usize) -> Result<()> {
        if new > self.imgsz {
            let ptr = unsafe {
                let ptr: *mut u8 = libc::realloc(self.img as *mut libc::c_void, new) as *mut u8;
                if ptr.is_null() {
                    return Err(anyhow::anyhow!(format!("failed to allocate memory for processus image, require minimal size: {}", new)));
                }
                ptr
            };
            self.img = ptr;
            self.imgsz = new;
        }
        Ok(())
    }

    /*
     * Read whole file then return a u8 vector buffered
    */
    fn read_whole_file(file: &String) -> std::io::Result<Vec<u8>> {
        let mut file = File::open(file)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(data)
    }

    /* 
     * Load EHDR/PHDR/SHDR into image
    */
    fn load_static_hdrs(&mut self, binobj: &goblin::elf::Elf) -> Result<()> {
        self.ToReallocOrNoToRealloc(binobj.header.e_ehsize as usize)?;
        let ehdr = self.img as *mut u8;
        unsafe {
            for i in 0..(binobj.header.e_ehsize as usize) {
                std::ptr::write(ehdr.wrapping_add(i), self.buf[i]);
            }
        }
        self.ToReallocOrNoToRealloc((binobj.header.e_phoff as usize) + (binobj.header.e_phentsize * binobj.header.e_phnum) as usize)?;
        let phdr = self.img.wrapping_add(binobj.header.e_phoff as usize);
        let add = binobj.header.e_phoff as usize;
        unsafe {
            for i in 0..((binobj.header.e_phentsize * binobj.header.e_phnum) as usize) {
                std::ptr::write(phdr.wrapping_add(i), self.buf[i + add]);
            }
        }
        self.ToReallocOrNoToRealloc((binobj.header.e_shoff as usize) + (binobj.header.e_shentsize * binobj.header.e_shnum) as usize)?;
        let shdr = self.img.wrapping_add(binobj.header.e_shoff as usize);
        let add = binobj.header.e_shoff as usize;
        unsafe {
            for i in 0..((binobj.header.e_shentsize * binobj.header.e_shnum) as usize) {
                std::ptr::write(shdr.wrapping_add(i), self.buf[i + add]);
            }
        }
        Ok(())
    }

    /*
     * Get section index from name
    */
    fn section_get_index_from_name(&mut self, binobj: &goblin::elf::Elf, sec: &mut String) -> Result<usize> {
        let shdrtab = binobj.section_headers[binobj.header.e_shstrndx as usize].sh_offset;
        sec.push(0x0 as char);
        for idx in 0..binobj.section_headers.len() {
            unsafe {
                if libc::strcmp(
                            (self.buf.as_ptr() as u64 + shdrtab + binobj.section_headers[idx].sh_name as u64) as *const i8,
                            sec.as_ptr() as *const i8
                        ) == 0 {
                    return Ok(idx)
                }
            }
        }
        Err(anyhow::anyhow!("Invalid section requested"))
    }

    /*
     * Load section at their respective addr
    */
    fn load_sections(&mut self, binobj: &goblin::elf::Elf) -> Result<()> {
        for shdr in &binobj.section_headers {
            let off = match shdr.sh_addr {
                0 => shdr.sh_offset as usize,
                _ => shdr.sh_addr as usize,
            };
            let tgt = self.img.wrapping_add(off);
            unsafe {
                for i in 0..(shdr.sh_size as usize) {
                    std::ptr::write(tgt.wrapping_add(i), self.buf[off + i]);
                }
            }
        }
        Ok(())
    }

    /*
     * Load a SO, for now, we don't load recursively, IT'S ABSOLUTELY NEEDED
    */
    fn load_so(&mut self, sof: &str) -> Result<usize> {
        let sop = &format!("{}{}", DYNAMIC_LIBRARY_PATH_LINUX, sof);
        let path = std::path::Path::new(&sop);
        let buf = std::fs::read(path).with_context(|| format!("Failed to read the given path: {:?}", path))?;
        let soobj = goblin::elf::Elf::parse(&buf).with_context(|| format!("Invalid ELF format: {:?}", path))?;
        let load = self.imgsz;
        self.ToReallocOrNoToRealloc(self.imgsz + buf.len())?;
        unsafe {
            libc::memcpy(self.img.wrapping_add(load) as *mut libc::c_void, buf.as_ptr() as *const libc::c_void, buf.len());
        }
        log!(format!("Loading SO {} at address {:#X}", sop, load));
        Ok(load)
    }

    /*
     * Load resolve reloc & load dynamic libraries
    */
    fn load_resolve_dynamic(&mut self, binobj: &goblin::elf::Elf) -> Result<()> {
        let dynamic = &binobj.dynamic;
        let dynstr  = &binobj.dynstrtab;
        let relaplt = &binobj.pltrelocs.to_vec();
        let dynrel  = &binobj.dynrelas;
        let mut sovecstr: Vec<&str>   = Vec::new();
        let mut sovecload: Vec<usize> = Vec::new();
        /* host all sym relocs we have to do */
        let mut dynsymvec: Vec<goblin::elf::reloc::Reloc> = Vec::new();
        for rela in dynrel {
            if rela.r_sym != 0 {
                dynsymvec.push(rela);
            }
        }
        for rel in relaplt {
            dynsymvec.push(*rel);
            // println!("{:?}", &dynstr[binobj.dynsyms.to_vec()[rel.r_sym].st_name]);
        }
        println!("{:?}", dynsymvec);
        for get in &dynamic.as_ref().unwrap().dyns {
            if get.d_tag == 1 {
                sovecstr.push(&dynstr[get.d_val as usize]);
                sovecload.push(self.load_so(&dynstr[get.d_val as usize])?);
            }
        }
        Ok(())
    }

    fn dump_image(&self) -> Result<()> {
        let mut f = File::create("output.img")?;
        let mut vec = Vec::new();
        unsafe {
            for i in 0..self.imgsz {
                vec.push(std::ptr::read(self.img.wrapping_add(i)));
            }
        }
        f.write_all(&vec)?;
        Ok(())
    }

    /*
     * from binary to image then return the entry point
    */
    pub fn load(&mut self) -> Result<usize> {
        let bufcp = &self.buf.clone();
        let binobj = goblin::elf::Elf::parse(bufcp)?;
        self.load_static_hdrs(&binobj)?;
        self.load_sections(&binobj)?;
        self.load_resolve_dynamic(&binobj)?;
        self.dump_image()?;
        log!(format!("Process image loaded: size -> {:#X} | addr -> {:#X}", self.imgsz, self.img as usize));
        Ok(binobj.header.e_entry as usize)
    }

    // pub fn SymAddrFromName(&self) {}
    // pub fn SymNameFromAddr(&self) {}
}