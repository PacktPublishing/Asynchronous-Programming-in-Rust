# Corofy

This tool aims to explain a few things about coroutines, and
by extension, async/await in Rust.

The tool will take an input file like provided below and re-write
the coroutine-syntax to state machines, which is very similar to
what happens when you use the async/await syntax in Rust.

This way we can create examples easier and change them around and
see what happens to the generated code.

The tool will not remove anything from the file, but it will comment out
the functions marked with `coroutine`, and re-arrange the transformation of them so
that they're placed last in the file.

The output will both contain the original `coroutine` function (commented out), and
comments explaining where the different parts of the original code is in the
state machine.

## Installation

You can install locally on your computer to run the examples via the command line by
entering the crate folder and writing:

```

cargo install --path .
```

You can avoid installing the project by using a more contrived syntax when using the tool
to rewrite the example files like this:

```

cargo run -- [path-to-file] [optional-out-path]
```

## Note

This tool is very brittle. I can only guarantee it will correctly re-write the exact
examples we go through in the book and in this repository. To name a few things it won't
support:

- Leaf coroutines (the ones created by Http::get) that returns anything else than `Future<Output=String>` (however, it should be possible with relatively minor effort to rewrite it to at least return `Future<Output=Vec<u8>>` to give it slightly more flexibility. Have a go if you want to!)
- Non-leaf coroutines that return anything else than `Future<Output=()>`
- Borrowing across wait points of any kind
- any variable or function names containing the word "coroutine" will cause the program to fail, it must only be used in front of the specific functions that you want to re-write (however, writing the word `coroutine` in a comment should be fine :))
- Oh, and all futures have an Output type of `String` even if they don't return anything. This simplifies the code a bit, and without access to type information outside of the file we're parsing, we can't really rely on getting the types correctly anyway.
- and much, much more

## Why don't you implement this as a macro instead?

Using procedural macros would be a preferred way to solve this for more serious use.
The only downside with macros, is that it's not so easy to see the generated code which
is one of the key points for us. We want to inspect the transformations so we
can learn from them.

We'll also rely on `corofy` to generate the boilerplate for
our state machines in the next chapters so we can expand on them manually to learn
about `Waker` and `Pin`. This would not be possible if we used macros.

There is already a macro implementation used for prototyping async/await. You can take a look at [https://github.com/alexcrichton/futures-await](https://github.com/alexcrichton/futures-await) if you want to see an example of how this can be implemented using macros.

## Usage

```
corofy [src_path] [optional-dest-path]
```

If no destination path is provided, it will default to writing to the same
directory where the src file is located and adding the postfix "_corofied" to the
file name.

## Detailed explanation

When installed you can give it a file using normal Rust code and our
own coro/wait syntax. The code below:

```rust

mod http;
mod future;

use future::*;
use crate::http::Http;

coro fn read_request(i: usize) {
    let path = format!("/{}/HelloWorld{i}", i * 1000);
    let txt = Http::get(&path).wait;
    println!("{txt}");
}

coro fn async_main() {
    println!("Program starting");
    let mut futures = vec![];

    for i in 0..5 {
        futures.push(read_request(i));
    }

    let txt = future::join_all(futures).wait;
    println!("{txt}");
}



fn main() {
    let mut future = async_main();

    loop {
        match future.poll() {
            PollState::NotReady => {
                println!("NotReady");
                // call executor sleep
                thread::sleep(Duration::from_millis(200));
            }

            PollState::Ready(_) => break,
        }
    }
}
```

...will be re-arrange and re-writen to this:

```rust
use std::{
    thread,
    time::Duration,
};

mod http;
mod future;

use future::*;
use crate::http::Http;







fn main() {
    let mut future = async_main();

    loop {
        match future.poll() {
            PollState::NotReady => {
                println!("NotReady");
                // call executor sleep
                thread::sleep(Duration::from_millis(200));
            }

            PollState::Ready(_) => break,
        }
    }
}

// =================================
// We rewrite this:
// =================================

// coro fn read_request(i: usize) {
//     let path = format!("/{}/HelloWorld{i}", i * 1000);
//     let txt = Http::get(&path).wait;
//     println!("{txt}");

// }

// =================================
// Into this:
// =================================

fn read_request(i: usize) -> impl Future<Output=()> {
    Coroutine0::new((i))
}

enum State0 {
    Start(usize),
    Wait1(Box<dyn Future<Output = String>>),
    Resolved,
}

struct Coroutine0 {
    state: State0,
}

impl Coroutine0 {
    fn new(i: usize) -> Self {
        Self { state: State0::Start(i) }
    }
}


impl Future for Coroutine0 {
    type Output = ();

    fn poll(&mut self) -> PollState<()> {
        match self.state {
            State0::Start(i) => {
                // ---- Code you actually wrote ----
                let path = format!("/{}/HelloWorld{i}", i * 1000);

                // ---------------------------------
                let fut1 = Box::new( Http::get(&path));
                self.state = State0::Wait1(fut1);
                PollState::NotReady
            }

            State0::Wait1(ref mut f1) => {
                match f1.poll() {
                    PollState::Ready(txt) => {
                        // ---- Code you actually wrote ----
                        println!("{txt}");

                        // ---------------------------------
                        self.state = State0::Resolved;
                        PollState::Ready(())
                    }
                    PollState::NotReady => PollState::NotReady,
                }
            }

            State0::Resolved => panic!("Polled a resolved future")
        }
    }
}


// =================================
// We rewrite this:
// =================================

// coro fn async_main() {
//     println!("Program starting");
//     let mut futures = vec![];
//
//     for i in 0..5 {
//         futures.push(read_request(i));
//     }
//
//     let txt = future::join_all(futures).wait;
//     println!("{txt}");

// }

// =================================
// Into this:
// =================================

fn async_main() -> impl Future<Output=()> {
    Coroutine1::new()
}

enum State1 {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Resolved,
}

struct Coroutine1 {
    state: State1,
}

impl Coroutine1 {
    fn new() -> Self {
        Self { state: State1::Start }
    }
}


impl Future for Coroutine1 {
    type Output = ();

    fn poll(&mut self) -> PollState<()> {
        match self.state {
            State1::Start => {
                // ---- Code you actually wrote ----
                println!("Program starting");
    let mut futures = vec![];

    for i in 0..5 {
        futures.push(read_request(i));
    }


                // ---------------------------------
                let fut1 = Box::new( future::join_all(futures));
                self.state = State1::Wait1(fut1);
                PollState::NotReady
            }

            State1::Wait1(ref mut f1) => {
                match f1.poll() {
                    PollState::Ready(txt) => {
                        // ---- Code you actually wrote ----
                        println!("{txt}");

                        // ---------------------------------
                        self.state = State1::Resolved;
                        PollState::Ready(())
                    }
                    PollState::NotReady => PollState::NotReady,
                }
            }

            State1::Resolved => panic!("Polled a resolved future")
        }
    }
}
```

## Final note

The whole program bear clear signs that I originally thought this would only
be a few lines of code to support the very limited cases we cover in these
examples. Apparently, that's not the case but it's not any goal for me to
improve this beyond proving the points I try to make in the book.

If you ever want to implement a more robust solution, using procedural macros is
the way to go.