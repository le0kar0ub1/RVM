global _start

section .text

_start:
    mov rcx, 0x4
    mov rax, wrme
    inc BYTE [rax]
re:
    mov rax, 0x1
    mov rdi, 0x1
    mov rsi, wrme
    mov rdx, 0xD
    syscall
    dec rcx
    cmp rcx, 0x0
    jne re

    mov rax, 0x3C
    mov rdi, 0x67
    syscall

section .data
   wrme DB 'hello world!', 0xA
