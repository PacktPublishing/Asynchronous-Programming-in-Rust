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
since it works on both platforms), you'll notice that we set a slightly
different name when the function is exported on MacOS and even though we
write `#[no_mangle]`, the compiler prepends the name of the function with an
underscore `_`.

This is due to the platform ABI on MacOS which expects functions to be prepended
with `_`. So, when we call the function in our inline assembly later on
we need to account for that. For example by changing:

```rust
asm!("call switch", in("rdi") old, in("rsi") new, clobber_abi("C"));
```

To:

```rust
asm!("call _switch", in("rdi") old, in("rsi") new, clobber_abi("C"));
```

Another workaround is to strip the first byte of the name on
export by adding the attribute below to the `switch` function:

```rust
#[cfg_attr(target_os = "macos", export_name = "\x01switch")]
```

You can read more about this here: https://github.com/rust-lang/rust/issues/35052#issuecomment-235420755
