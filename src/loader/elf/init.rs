extern crate elf;

use std::path::PathBuf;

pub fn init(file: &String) {
    let path = PathBuf::from(file);
    let binobj = elf::File::open_path(&path).expect("Invalid given executable");
    match binobj.ehdr.machine {
        elf::types::EM_X86_64 => (),
        elf::types::EM_AARCH64 => (),
        elf::types::EM_386 => (),
        _ => panic!("Invalid target architecture")
    }
    println!("elf initialized");
}