# Runtime Const Generics in rust

Const generics enable developers to conditionally execute debug code without runtime overhead.
This setting must be made at compile time.
This program demonstrates how const generic booleans can be changed at runtime.
It should be safe as long as the size of all struct members remain independent of the value of the const generic.

When a const generic is changed at runtime, its fat pointer is modified, so that it points to the appropriate vtable.
