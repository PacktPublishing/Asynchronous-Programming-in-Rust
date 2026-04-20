# c-fibers

This example is the exact same example as we went through in the book.

## Technical requirements

This example will only work correctly on Unix platforms running on
a x86-64 processor. Most desktop/laptop processors from Intel and AMD produced
the last decade will use this instruction set.
For Windows users this example will run fine in [WSL](https://learn.microsoft.com/en-us/windows/wsl/install).
Linux users can run the example directly. MacOS users can't run the example on
the newer M-family CPU's, but MacOS users running an intel-based Mac can
run the example just fine.

## Running the example

This example uses the feature "naked_functions" so we need to run it using
nightly Rust until https://github.com/rust-lang/rust/pull/134213 is released in
stable Rust. There are two ways to do that.

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
