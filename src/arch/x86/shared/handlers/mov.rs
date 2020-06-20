use iced_x86::*;

pub fn mov_handler(instr: Instruction) {
    println!("{:?}", instr);
    println!("{:?} {:?}", instr.op_register(0), instr.op_register(1));
}