`src/mem`
=========

Here is handled all the memory artefact.

| Component  | Description                                         |
|------------|-----------------------------------------------------|
| `stack`    | supervisor provided stack while loading the program |
| `segments` | program memory description with flaged segmentation |

All the memory hits are interfaced here, we need to verify that the accesses segment is valid to avoid segmentation faults.

The program executed is runnning in the heap of `RVM` which works like a supervisor.

Then, the execution environnement provided is fully securited.