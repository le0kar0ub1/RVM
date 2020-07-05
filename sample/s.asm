global _start

section .text

_start:
    mov rcx, 0x4
    mov rax, wrme
    xor BYTE [rax], 0x10
    mov rax, 0x0
re:
    mov rax, 0x1
    mov rdi, 0x1
    mov rsi, wrme
    mov rdx, 0xD
    syscall
    sub rcx, 0x1
    cmp rcx, 0x0
    jne re

    mov rax, 0x3C
    mov rdi, 0x67
    syscall

section .data
   wrme DB 'hello world!', 0xA
