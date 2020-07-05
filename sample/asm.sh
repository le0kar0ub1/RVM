#!/bin/sh

function check() {
    if [ $? -ne 0 ]; then
        exit 1
    fi
}

nasm -f elf64 $1 -o tmp.o
check
ld tmp.o -o $(echo $1 | cut -d . -f 1)
check
rm tmp.o
