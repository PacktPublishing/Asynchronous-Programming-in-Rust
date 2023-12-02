use std::{marker::PhantomPinned, pin::Pin};

// use std::pin::pin;
// fn main() {
//     let mut x = pin!(MaybeSelfRef::default());
//     MaybeSelfRef::init(x.as_mut());
//     println!("{:?}", x.as_ref().b());
// }

// // Pinning stack
// fn main() {
//     let mut x = MaybeSelfRef::default();
//     let mut x = unsafe { Pin::new_unchecked(&mut x) };
//     MaybeSelfRef::init(x.as_mut());
//     println!("{:?}", x.as_ref().b());
// }

fn main() {
    let mut x = Box::pin(MaybeSelfRef::default());
    x.as_mut().init();
    let b = x.as_mut().b().unwrap();
    println!("{}", b);
    *b = 2;
    println!("{x:?}");
}

#[derive(Default, Debug)]
struct MaybeSelfRef {
    a: usize,
    b: Option<*mut usize>,
    _pin: PhantomPinned,
}

impl MaybeSelfRef {
    fn init(self: Pin<&mut Self>) {
        unsafe {
            let Self { a, b, .. } = self.get_unchecked_mut();
            *b = Some(a);
        }
    }

    fn b(self: Pin<&mut Self>) -> Option<&mut usize> {
        unsafe { self.get_unchecked_mut().b.map(|b| &mut *b) }
    }
}
