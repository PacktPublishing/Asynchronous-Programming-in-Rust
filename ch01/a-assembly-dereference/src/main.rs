//! # FIXES:
//! The number is identical to the number in the GitHub issue tracker
//!
//! ## FIX ISSUE #11:
//! See:https://github.com/PacktPublishing/Asynchronous-Programming-in-Rust/issues/11
//! The book didn't make it clear that this example will only work on `x86-64` architecture,
//! so users on newer M-series macs (which uses the ARM64 instruction set), will get a
//! compilation error. This is solved by conditionally compiling a version that works
//! with the  ARM64 instruction set.

use std::arch::asm;

fn main() {
    let t = 100;
    let t_ptr: *const usize = &t; // if you comment out this...
    // ...and uncomment the line below. The program will fail.
    // let t_ptr = 99999999999999 as *const usize;
    let x = dereference(t_ptr);

    println!("{}", x);
}

#[cfg(target_arch = "x86_64")]
fn dereference(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe {
        asm!("mov {0}, [{1}]", out(reg) res, in(reg) ptr)
    };
    res
}

// FIX #11
#[cfg(target_arch = "aarch64")]
fn dereference(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe {
        asm!("ldr {0}, [{1}]", out(reg) res, in(reg) ptr)
    };
    res
}


