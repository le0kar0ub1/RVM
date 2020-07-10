# RVM

A simple rust (micro-)'virtual machine'.

RVM run binary and provide a securited run-time environnement for userspace programs.

[root documentation](doc/root.md)

## Dependencies

* cargo

## Build

Assured by our servitor `cargo`.

`cargo build`

## Run

`caro run $BINARY`

where `$BINARY` is the executable path you want run.

## Sub-Ref

| Directory     | Ref                   |
|---------------|-----------------------|
| `src/arch`    | [ref](src/arch)       |
| `src/loader`  | [ref](src/loader)     |
| `src/mem`     | [ref](src/mem)        |

## About

Scoped architecture: `x86_64`

## Sample

Juste a sample of x64 assembly code ot test the VM.

```assembly
global _start

section .text

called:
    mov rax, 0x1
    mov rdi, 0x1
    lea rsi, [metoo]
    mov rdx, 0xF
    syscall
    ret

_start:
    lea rax, [wrme]
    inc BYTE [rax]
    mov rax, called
    call rax
    mov rcx, 0x4
re:
    mov rax, 0x1
    mov rdi, 0x1
    lea rsi, [wrme]
    mov rdx, 0xD
    push rcx
    syscall
    pop rcx
    dec rcx
    cmp rcx, 0x0
    jne re

    mov rax, 0x3C
    mov rdi, 0x0
    syscall

section .data
   wrme DB 'gello world!', 0xA
   metoo DB 'called routine', 0xA
```

```shell
[anywhere/RVM]$ sample/asm.sh sampe/s
[anywhere/RVM]$ cargo run sample/s
```

Then the given output:

```
called routine
hello world!
hello world!
hello world!
hello world!
Program exit with code: 0
```

## Epilogue

To learn rust language.

Feel free to fork, use, improve.