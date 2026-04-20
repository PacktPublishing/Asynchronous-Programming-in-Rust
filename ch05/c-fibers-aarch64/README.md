# c-fibers-aarch64

This example is an adaptation of `c-fibers` to AArch64 architecture.

## Technical requirements

This example will only work correctly on Unix platforms running on
a AArch64 processor.

## Running the example

This example uses the unstable feature "naked_functions" so we need to run it
using nightly Rust. There are two ways to do that.

1. Tell cargo to use the nightly toolchain when you run the program:

```
cargo +nightly run
```

2. Override the default toolchain for this directory:

```
rustup override set nightly
cargo run
```

## Safety

The implementation is very unsafe and only focuses on the bare minimum to get a working example running. We focus on explaining the concepts, and the only focus is on explaining them as simple as I can.

While a fiber implementation like this will never be possible to do fully in safe Rust, there are many ways to make it safer, and it's a good readers excercise to do so. Just beware that you might have to change the API somewhat since some of the unsafe parts of this example is there just to make the API very easy to understand for learning purposes.
