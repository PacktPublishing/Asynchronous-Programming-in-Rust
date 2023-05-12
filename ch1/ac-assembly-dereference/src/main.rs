
use std::arch::asm;

fn main() {
    let t = 100;
    let t_ptr: *const usize = &t; // if uou comment out this...
    // ...and uncomment the line below. The program will fail.   
    // let t_ptr = 99999999999999 as *const usize;
    let x = dereference(t_ptr);

    println!("{}", x);
}

fn dereference(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe { 
        asm!("mov {0}, [{1}]", out(reg) res, in(reg) ptr)
    };
    res
}
