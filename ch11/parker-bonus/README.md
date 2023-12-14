# Parker

As we explained in the book, relying on `thread::park` alone
is not a good solution since "everyone" can use `thread::current`
and `thread::park` to implement simple synchronization in their code.

By doing so they can cause us to miss wakeups or to simply deadlock
since we rely on the same mechanism for parking our executor.

Since it doesn't require many lines of code to create a working solution ourselves we'll show how
we can solve that by using a `Condvar` and a `Mutex` instead, but there are also libraries
that does this for you. One of the popular ones is the [Parker](https://docs.rs/crossbeam/latest/crossbeam/sync/struct.Parker.html) provided by the crossbeam crate.

If you want to write one yourself, it can be as simple as this:

```rust, ignore
#[derive(Default)]
struct Parker(Mutex<bool>, Condvar);

impl Parker {
    fn park(&self) {

        // We aquire a lock to the Mutex which protects our flag indicating if we
        // should resume execution or not.
        let mut resumable = self.0.lock().unwrap();

            // We put this in a loop since there is a chance we'll get woken, but
            // our flag hasn't changed. If that happens, we simply go back to sleep.
            while !*resumable {

                // We sleep until someone notifies us
                resumable = self.1.wait(resumable).unwrap();
            }

        // We immidiately set the condition to false, so that next time we call `park` we'll
        // go right to sleep.
        *resumable = false;
    }

    fn unpark(&self) {
        // We simply acquire a lock to our flag and sets the condition to `runnable` when we
        // get it.
        *self.0.lock().unwrap() = true;

        // We notify our `Condvar` so it wakes up and resumes.
        self.1.notify_one();
    }
}
```

The `Condvar` in Rust is designed to work together with a Mutex. Usually, you'd think that we don't
release the mutex-lock we acquire in `self.0.lock().unwrap();` before we go to sleep. Which means
that our `unpark` function never will acquire a lock to our flag and we deadlock.

Using `Condvar` we avoid this since the `Condvar` will consume our lock so it's released at the
moment we go to sleep.

When we resume again, our `Condvar` returns our lock so we can continue to operate on it.

## Usage

```rust
use std::{thread, time, sync::{Arc, atomic::{AtomicBool, Ordering}}};

let flag = Arc::new(AtomicBool::new(false));
let parker = Arc::new(Parker::default());

let flag_clone = flag.clone();
let parker_clone = parker.clone();

thread::spawn(move || {
    thread::sleep(time::Duration::from_millis(200));
    flag_clone.store(true, Ordering::SeqCst);
    parker_clone.unpark();
});
assert!(!flag.load(Ordering::SeqCst), "Flag should be false at this point!");
parker.park();
assert!(flag.load(Ordering::SeqCst), "Flag should be true at this point!");
```
