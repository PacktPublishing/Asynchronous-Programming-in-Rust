# How to run these examples on a Mac with an M-series chip

Newer Macs use a chip using the ARM ISA, which won't work out of the box with
these examples. However, these Macs ship with an emulation software called Rosetta
we can use to emulate a x86-64 instruction set. With a few simple steps we can
cross compile the examples to target x86-64 Macs and run them using Rosetta:

1. Switch the architecture on the terminal in one of two ways:
   1. write `$env /usr/bin/arch -x86_64 /bin/zsh --login` in the terminal to force
   the current session to emulate a x86-64 architecture.
   2. Find "Terminal.app", left click and choose "info". Check the box
   "Open with Rosetta" (remember to uncheck later if you don't want Terminal to always
   open using Rosetta).
2. Confirm that you're now using Rosetta to emulate x86-64 by typing the command: `arch`.
It should report `i386`.
3. Run `rustup target add x86_64-apple-darwin`
4. You can now run the examples using the command: `cargo run --target x86_64-apple-darwin`

## Important

In the example: "c-fibers" and "d-fibers-closure" (and "e-fibers-windows"
since it works on both platforms), you need to make a small change. The line:

```rust
asm!("call switch", in("rdi") old, in("rsi") new, clobber_abi("C"));
```
Needs to be changed to:

```rust
asm!("call _switch", in("rdi") old, in("rsi") new, clobber_abi("C"));
```

The only change is the added `_` before `switch`
