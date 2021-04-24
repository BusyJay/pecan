### Why not put structs under packages?

There are many considerations, the most important one is Rust's module
doesn't work like namespace in C++. Rust's module normally couple with
file directory, and module from different crate (or even different
files) can never mixed together without manual effort. If manual effort
is acceptable, then it doesn't matter whether put them into packages in
the first place.

If just using the proto directory instead, it's easier for compiler to
generate code, and the result is more predictable. Just using proto
directory is also a convention used by Python version, which has similar
module problem with Rust.
