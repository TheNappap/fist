# Fist

> [!WARNING]  
> This is an unstable project and needs nightly rust to compile.

Fixed sized trait objects (FiSTs) are an alternative structure for boxed trait objects and enums in Rust.
Its goal is to balance the flexibility of trait objects with the efficiency of enums.
* Flexibility: fists can be any object implementing the trait like regular trait objects with the limitation of the size of the object (see DynFists below).
* Efficiency: data of fists is stored on stack like enums. However it still needs a pointer to a vtable like regular trait objects.

For more flexibility, but lower efficiency, there are also Dynamic FiSTs (DynFist).
DynFists dynamically allocate on stack or heap depending on the size of the object.
It is basically a hybrid of a FiST for object within the chosen size and a boxed trait object otherwise.

### Benchmarks

Benchmarks comparing different approaches for initialization and calling.
Fists and DynFists improve on initialization time, compared to boxes, but not on calling.
This can be explained by the fact that fists use stack memory, but for calling it still needs a vtable.

|                | init_small | init_big | call  |
| ---            | ---        | ---      | ---   |
| box            | 45.494     | 52.178   | 3.837 |
| enum           | 0.468      | 29.496   | 0.522 |
| dyn_fist_heap  | /          | 49.338   | 3.759 |
| dyn_fist_stack | 1.041      | 31.913   | 3.755 | 
| fist           | 0.713      | 30.453   | 3.793 |
