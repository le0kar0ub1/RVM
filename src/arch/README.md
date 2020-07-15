`src/arch`
==========

This directory host the arch-specific targets.

Currently only `x86_64` is handled.

|  Path                                 | Job                                                              |
|---------------------------------------|------------------------------------------------------------------|
| `x86/x86_64/alu.rs`                   | fully virtual ALU, in fact just used for updating cpu flags      |
| `x86/x86_64/cpu.rs`                   | x64 full CPU registers                                           |
| `x86/x86_64/scheduler.rs`             | read instruction from image, pass instruction update RIP         |
| `x86/x86_64/opcode_handler.rs`        | opcode redirection to instruction handler                        |
| `x86/x86_64/syscalls.rs`              | x64 syscalls handler & map                                       |
| `x86/x86_64/handlers.rs`              | instruction handlers                                             |