# d-fibers-closure

This is a slightly modified version that mimics the API of the standard `thread::spawn` API from the standard library.

Instead of just accepting a function pointer, our spawn function now accepts
a `FnOnce` just like `std::thread::spawn` does.

The example is slightly more involved and as the topic is rather complex I added this as an example for further inspiration instead of using it as our main example.

## Safety

The implementation is wildly unsafe and onlyc focuses on the bare minimum to get a working example running. We focus on explaining the concepts, not best practices. 

While a fiber implementation like this will never be possible to do fully in safe Rust, there are many ways to make it safer, and it's a good readers excercise to do so. Just beware that you might have to change the API somewhat to make it safer since mutating global statics using raw pointers (and any mutating of what Rust assumes are exclusive borrows using the tricks we do here) are a sure way to rely on undefined behavior and this example is no exception to that.