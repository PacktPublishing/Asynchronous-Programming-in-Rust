# Thoughts

- After converting to std::future::Future which is not much work:
- Implement server.rs with a simple listen function
- Can only accept delay requests like the ones we make
- Implement a simple timer reactor using
  - A thread
  - Veddeque to store timers/wakers
  - thread::park_timeout() / thread unpark to add new/wake up and call wake once a timer has expired

We now have a client and server running our own async runtime single threaded. How many "threads" can we achieve? More than OS threads on a sinlge thread?

Or is this too much work for readers to do?
