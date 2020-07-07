global _start

section .text

called:
    ret

_start:
    mov rcx, 0x4
    lea rax, [wrme]
    inc BYTE [rax]
    mov rax, called
    call rax
re:
    mov rax, 0x1
    mov rdi, 0x1
    mov rsi, wrme
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
