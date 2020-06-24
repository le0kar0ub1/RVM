# RVM Documentation

RVM is a micro-VM wroten in `rust` language.

RVM handle (in part) dynamic linking.

There are three side:
| Part/path                  | Job                                                                                 |
-----------------------------|-------------------------------------------------------------------------------------|
| [loader](../src/loader)    | create an image process from the given executable file                              |
| [mem](../src/mem)          | memory handling, component and operation on memory with virtual address translation |
| [arch](../src/arch)        | target-specific objects as: processor, scheduler, instructions, ....                |

