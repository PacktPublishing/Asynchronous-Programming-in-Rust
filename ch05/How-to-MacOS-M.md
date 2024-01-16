# How to run these examples on a Mac with an M-seriXes chip

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

## Note

In the example: "c-fibers" and "d-fibers-closure" (and "e-fibers-windows"
since it works on both platforms), you'll notice that we use a conditional
compilation attribute for `macos` targets.

This is due to the platform ABI on MacOS which expects functions to be prepended
with `_`. So, LLVM will by default add an underscore to our functions even if we
tag them with `#[no_mangle]`.

The workaround we use is to strip the first byte of the name that's exported
by LLVM on MacOS by adding the attribute below to the `switch` function:

```rust
#[cfg_attr(target_os = "macos", export_name = "\x01switch")]
```

You can read more about this here: <https://github.com/rust-lang/rust/issues/35052#issuecomment-235420755>
