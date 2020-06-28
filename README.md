# RVM

A simple rust 'virtual machine'.

In fact, this is just a userspace program execution.

[root documentation](doc/root.md)

## Dependencies

* cargo
* rustc

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

## Epilogue

To learn rust language.

Feel free to fork, use, improve.