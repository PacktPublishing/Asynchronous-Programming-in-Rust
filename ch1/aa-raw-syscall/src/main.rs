use std::arch::asm;

fn main() {
    let message = String::from("Hello world from interrupt!\n");
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
// macOS raw syscall
// ----------------------------------------------------------------------------

#[cfg(target_os = "macos")]
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
// Windows raw syscall
// ----------------------------------------------------------------------------
#[cfg(target_os = "windows")]
#[inline(never)]
fn syscall(message: String) {
    panic!("We can't. Windows doesn't have a stable syscall ABI")
}
