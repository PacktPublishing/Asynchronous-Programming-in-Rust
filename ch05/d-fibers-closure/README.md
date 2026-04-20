# d-fibers-closure

This is a slightly modified version that mimics the API of the standard `thread::spawn` API from the standard library.

Instead of just accepting a function pointer, our spawn function now accepts
a `FnOnce` just like `std::thread::spawn` does.

The example is slightly more involved and as the topic is rather complex I added this as an example for further inspiration instead of using it as our main example.

This example is the exact same example as we went through in the book with some minor changes. I've made a comment on the places where the example has been changed to make it easier for you to see exactly what has been changed from
the original one we covered in the book.

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

The implementation is wildly unsafe and only focuses on getting a working example running.
The focus is on explaining the concepts as simply as possible and not best practices.

While a fiber implementation like this will never be possible to do fully in safe Rust, there are many ways to make it safer, and it's a good readers exercise to do so. Just beware that you might have to change the API somewhat to make it safer since mutating global statics using raw pointers (and any mutating of what Rust assumes are exclusive borrows using the tricks we do in the `call` function) are a sure way to get undefined behavior and this example is no exception to that.
