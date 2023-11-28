use rand::prelude::*;

fn main() {
    selfref1();
}

fn selfref1() {
    let mut first = SelfRef1::new(0);
    println!("It's OK: {}", first.b());
    let second = std::mem::replace(first.as_mut(), *SelfRef1::new(2));
    println!("Ooops: {}", first.b());
    println!("Ooops: {}", second.b());
}

#[derive(Debug)]
struct SelfRef1 {
    a: [char;3],
    b: *const char,
}

impl SelfRef1 {
    fn new(n: usize) -> Box<Self> {
        let mut selfref = Box::new(Self {
            a: ['a', 'b', 'c'],
            b: std::ptr::null(),
        });
        selfref.b = &selfref.a[n] as *const _;
        selfref
    }

    fn b(&self) -> &char {
        unsafe { &*self.b }
    }
}

// =======================================================
use std::pin::Pin;

fn selfref2() {
    let mut first = SelfRef2::new(0);
    println!("It's OK: {}", first.b());
    let mut second = SelfRef2::new(2);
    std::mem::swap(&mut first, &mut second);
    println!("Ooops: {}", first.b());
}

#[derive(Debug)]
struct SelfRef2 {
    a: [char;3],
    b: *const char,
    _pin: std::marker::PhantomPinned,
}

impl SelfRef2 {
    fn new(n: usize) -> Pin<Box<Self>> {
        let mut selfref = Box::pin(Self {
            a: ['a', 'b', 'c'],
            b: std::ptr::null(),
            _pin: std::marker::PhantomPinned,
        });
        //selfref.b = &selfref.a[n] as *const _;
        selfref
    }

    fn b(&self) -> &char {
        unsafe { &*self.b }
    }
}
