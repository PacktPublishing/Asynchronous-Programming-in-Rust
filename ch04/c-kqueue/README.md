# c-kqueue

This create contains a kqueue adaptation of the example code for chapter 4.
It should work if you're running on macOS or BSD, but I've only tested
on macOS.

Rather than attempt to duplicate, or re-explain the basic principles the
comments in this code attempt to explain the differences between the epoll
implementation and this one. My objective here was to enable the reader (and
myself) to understand this code in the context of the book rather than
standalone.

You can run the example by simply writing `cargo run`.

Otherwise see the [../a-epoll/README.md](../a-epoll/README.md) for further details.
