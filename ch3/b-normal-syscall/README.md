# Additional notes on the Windows Example

Now, just by looking at the code above you see it starts to get a bit more
complex, but let's spend some time to go through line by line what we do here as
well.

```rust, ignore
#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
```

The first line is just telling the compiler to only compile this if the `target_os` is Windows.

The second line is a linker directive, telling the linker we want to link to the library `kernel32` (if you ever see an example that links to `user32` that will also work).

```rust, ignore
extern "system" {
    /// https://docs.microsoft.com/en-us/windows/console/getstdhandle
    fn GetStdHandle(nStdHandle: i32) -> i32;
    /// https://docs.microsoft.com/en-us/windows/console/writeconsole
    fn WriteConsoleW(
        hConsoleOutput: i32,
        lpBuffer: *const u16,
        numberOfCharsToWrite: u32,
        lpNumberOfCharsWritten: *mut u32,
        lpReserved: *const std::ffi::c_void,
    ) -> i32;
}
```

First of all, `extern "system"`, tells the compiler that we will use the `system` calling convention, and is a little peculiar. On Windows you have different calling conventions whether you run 32-bit "x86" version of Windows (which uses "stdcall" calling convention), or 64-bit x86_64 version of Windows, which uses the "C" calling convention. The "system" calling convention will choose the right one based on the Windows version.

The next part is the functions we want to link to. On Windows, we need to link to two functions to get this to work: `GetStdHandle` and `WriteConsoleW`.
`GetStdHandle` retrieves a reference to a standard device like `stdout`.

`WriteConsole` comes in two flavours, `WriteConsoleW` that takes in Unicode text and `WriteConsoleA` that takes ANSI encoded text.

Now, ANSI encoded text works fine if you only write English text, but as soon as you write text in other languages you might need to use special characters that are not possible to represent in `ANSI` but is possible in `utf-8` and our program will break.

That's why we'll convert our `utf-8` encoded text to `utf-16` encoded Unicode codepoints that can represent these characters and use the `WriteConsoleW` function.

```rust, ignore
#[cfg(target_os = "windows")]
fn syscall(message: String) -> io::Result<()> {
    // let's convert our utf-8 to a format windows understands
    let msg: Vec<u16> = message.encode_utf16().collect();
    let msg_ptr = msg.as_ptr();
    let len = msg.len() as u32;

    let mut output: u32 = 0;
    let handle = unsafe { GetStdHandle(-11) };
    if handle == -1 {
        return Err(io::Error::last_os_error());
    }

    let res = unsafe { WriteConsoleW(handle, msg_ptr, len, &mut output, std::ptr::null()) };

    if res == 0 {
        return Err(io::Error::last_os_error());
    }

    assert_eq!(output, len);
    Ok(())
}
```

The first thing we do is to convert the text to utf-16 encoded text which
Windows uses. Fortunately, Rust has a built-in function to convert our `utf-8` encoded text to `utf-16` code points. `encode_utf16` returns an iterator over  `u16` code points that we can collect to a `Vec`.

```rust, ignore
let msg: Vec<u16> = message.encode_utf16().collect();
let msg_ptr = msg.as_ptr();
let len = msg.len() as u32;
```

Next, we get the pointer to the underlying buffer of our `Vec` and get the
length.

```rust, ignore
let handle = unsafe { GetStdHandle(-11) };
if handle  == -1 {
    return Err(io::Error::last_os_error())
}
```

The next is a call to `GetStdHandle`. We pass in the value `-11`. The values we
need to pass in for the different standard devices is actually documented
together with the `GetStdHandle` documentation:

| Handle | Value |
| ------ | ----- |
| Stdin  |   -10 |
| Stdout |   -11 |
| StdErr |   -12 |

Now we're lucky here, it's not that common that we find this information
together with the documentation for the function we call, but it's very convenient when we do.

The return codes to expect is also documented thoroughly for all functions so we handle potential errors here in the same way as we did for the Linux/macOS syscalls.

```rust, ignore
let res = unsafe {
    WriteConsoleW(handle, msg_ptr, len, &mut output, std::ptr::null())
};

if res == 0 {
    return Err(io::Error::last_os_error());
}
```

Next up is the call to the `WriteConsoleW` function. There is nothing too fancy about this.
