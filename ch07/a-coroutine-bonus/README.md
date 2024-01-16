# d-coroutine-bonus

This is just a bonus example showing that we get the
exact same program flow if we used the "proper" Future trait
and Rust's `async/await` syntax as we did when we hand wrote
our own state-machine in our first example `a-coroutine`.

In the original example we got:

```text
Program starting
FIRST POLL - START OPERATION
We can do other work!
We can do other work!
We can do other work!
We can do other work!
We can do other work!
We can do other work!
HTTP/1.1 200 OK
content-length: 11
connection: close
content-type: text/plain; charset=utf-8
date: Tue, 24 Oct 2023 20:35:09 GMT

HelloWorld1
FIRST POLL - START OPERATION
We can do other work!
We can do other work!
We can do other work!
We can do other work!
HTTP/1.1 200 OK
content-length: 11
connection: close
content-type: text/plain; charset=utf-8
date: Tue, 24 Oct 2023 20:35:09 GMT

HelloWorld2
```

Running this example and using the `std::future::Future`
trait and `async/await` we get:

```text
Program starting
FIRST POLL - START OPERATION
Schedule other tasks
Schedule other tasks
Schedule other tasks
Schedule other tasks
Schedule other tasks
Schedule other tasks
HTTP/1.1 200 OK
content-length: 16
connection: close
content-type: text/plain; charset=utf-8
date: Tue, 24 Oct 2023 20:38:37 GMT

HelloAsyncAwait1
FIRST POLL - START OPERATION
Schedule other tasks
Schedule other tasks
Schedule other tasks
Schedule other tasks
HTTP/1.1 200 OK
content-length: 16
connection: close
content-type: text/plain; charset=utf-8
date: Tue, 24 Oct 2023 20:38:37 GMT

HelloAsyncAwait2
```