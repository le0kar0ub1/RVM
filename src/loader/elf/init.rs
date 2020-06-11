extern crate elf;
extern crate libc;

use std::path::PathBuf;

#[derive(Debug)]
pub struct elfimg {
    img: *mut libc::c_void,
    bin: elf::File,
}

impl elfimg {
    pub fn new(file: &String) -> elfimg {
        let path = PathBuf::from(file);
        let binobj = elf::File::open_path(&path).expect("Invalid given executable");
        match binobj.ehdr.machine {
            elf::types::EM_X86_64 => (),
            elf::types::EM_AARCH64 => (),
            elf::types::EM_386 => (),
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
                panic!("failed to allocate memory");
            }
            libc::free(ptr as *mut libc::c_void);
            ptr
        };
        println!("elf parser initialized");
        elfimg {
            img: ptr,
            bin: binobj,
        }
    }

}