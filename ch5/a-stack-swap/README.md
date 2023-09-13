# a-stack-swap

This is the first example in chapter 5 showing the simplest
possible way to "trick" the CPU to use a stack we created
ourselves.

## Technical requirements

This example *should* work on all platforms using a x86-64 CPU.
Most modern Intel and AMD desktop/laptop CPU's use this
architecture. However, I recommend using a Unix based platforms like Linux
since that seems to work best.

The stack swap implementation is not complete (we will complete it in a later
example) and can report a strange error when exiting using `ctrl + c` (this
happens to me on Windows and if you leave it running it can take quite some
time for the program to exit).

## Running the example

You can run the example using `cargo run`

Since the example ends in a infinite loop, you'll have to quit
using `ctrl + c`