`src/mem`
=========

Here is handled all the memory artefacts.

| Component  | Description                                         |
|------------|-----------------------------------------------------|
| `stack`    | supervisor provided stack while loading the program |
| `segments` | program memory description with flaged segmentation |

All the memory hits are interfaced here, we need to verify that the accessed segment is valid to avoid segmentation faults.

The program executed is runnning on the `RVM` heap which works like a supervisor. If the program try to access a non-valid segment an error is raised and the execution is stopped.

So, the execution environnement provided is fully securited and the program can (should) not segfault.
In fact, there is one un-securited execution context, the systems calls. RVM does not implement self syscalls execution, then when one of them is triggered all the arguments are checked but the we let the system handle.