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

### How to solve cyclic struct if using `Option`?

recursive struct support is limited in proto2. Because pecan always inline
struct, so if struct is cyclic referenced, the struct's size can't be
calculated. Pecan provides a field option to allow put a field inside Box
to get around the problem. Just declare it like

```protobuf
syntax = "proto2";

import "pecan/options.proto";

message Test2 {
    optional int32 number = 1;
    // required Test1 te1 = 2 [(pecan.field_opt).box_field = true];
    optional Test1 te2 = 3 [(pecan.field_opt).box_field = true];
}

message Test1 {
    optional Test2 inner = 1;
}
```

The option only works for any singular message field in proto3 and
optional message field in proto2. It won't work for required field,
because pecan uses const function to initialize struct and there is no
way to initialize a Box in const function.

### Why use const function to initialize struct?

For fields missing values in proto3, it can be very handy to return a default
reference at every read accessor. So there should be an easy way to return a
default message. One way is always constructing a new struct at every get,
another way is using global value. Using LazyStatic should be possible, but I
prefer a more clean and zero cost way: use static directly with const fn.
