use std::arch::asm;

fn main() {
    let message = String::from("Hello world from raw syscall!\n");
    syscall(message);
}

// ----------------------------------------------------------------------------
// Linux raw syscall
// ----------------------------------------------------------------------------

#[cfg(target_os = "linux")]
#[inline(never)]
fn syscall(message: String) {
    let msg_ptr = message.as_ptr();
    let len = message.len();

    unsafe {
        asm!(
            "mov rax, 1",      // system call 1 is write on Linux
            "mov rdi, 1",      // file handle 1 is stdout
            "syscall",         // call kernel, software interrupt
            in("rsi") msg_ptr, // address of string to output
            in("rdx") len,     // number of bytes
            out("rax") _, out("rdi") _, lateout("rsi") _, lateout("rdx") _
        );
    }
}

// ----------------------------------------------------------------------------
// macOS raw syscall when running intel CPU (x86_64 architecture)
// ----------------------------------------------------------------------------

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
#[inline(never)]
fn syscall(message: String) {
    let msg_ptr = message.as_ptr();
    let len = message.len();

    unsafe {
        asm!(
            "mov rax, 0x2000004", // system call 0x2000004 is write on macos
            "mov rdi, 1",         // file handle 1 is stdout
            "syscall",            // call kernel, syscall interrupt
            in("rsi") msg_ptr,    // address of string to output
            in("rdx") len,         // number of bytes
            out("rax") _, out("rdi") _, lateout("rsi") _, lateout("rdx") _
        );
    }
}

// ----------------------------------------------------------------------------
// macOS raw syscall when running newer M familiy CPU (ARM 64 architecture)
// ----------------------------------------------------------------------------

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
#[inline(never)]
fn syscall(message: String) {
    let ptr = message.as_ptr();
    let len = message.len();

    unsafe {
        asm!(
            "mov x16, 4",
            "mov x0, 1",
            "svc 0",
            in("x1") ptr,
            in("x2") len,
            out("x16") _,
            out("x0") _,
            lateout("x1") _,
            lateout("x2") _
        );
    }
}

// ----------------------------------------------------------------------------
// Windows raw syscall
// ----------------------------------------------------------------------------
#[cfg(target_os = "windows")]
#[inline(never)]
fn syscall(message: String) {
    panic!("We can't. Windows doesn't have a stable syscall ABI")
}
