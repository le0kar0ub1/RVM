`src/arch`
==========

This directory host the arch-specific targets.

Currently only `x86_64` is handled.

|  Path                                                  | Job                                                              |
|--------------------------------------------------------|------------------------------------------------------------------|
| [x86/x86_64/alu](x86/x86_64/alu)                       | fully virtual ALU, in fact just used for updating cpu flags      |
| [x86/x86_64/cpu](x86/x86_64/cpu)                       | x64 full CPU registers                                           |
| [x86/x86_64/scheduler](x86/x86_64/scheduler)           | read instruction from image, pass instruction update RIP         |
| [x86/x86_64/opcode_handler](x86/x86_64/opcode_handler) | opcode redirection to instruction handler                        |
| [x86/x86_64/syscalls](x86/x86_64/syscalls)             | x64 syscalls handler & map                                       |
| [x86/x86_64/handlers](x86/x86_64/handlers)             | instruction handlers                                             |