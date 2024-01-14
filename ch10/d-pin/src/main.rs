use std::{
    marker::PhantomPinned,
    pin::{pin, Pin},
};

fn main() {
    heap_pinning();
    //stack_pinning_manual();
    //stack_pinning_manual_problem();
    //stack_pinning_macro();
    //pin_projection();
}

fn heap_pinning() {
    let mut x = Box::pin(MaybeSelfRef::default());
    x.as_mut().init();
    println!("{}", x.as_ref().a);
    *x.as_mut().b().unwrap() = 2;
    println!("{}", x.as_ref().a);
}

fn stack_pinning_manual() {
    let mut x = MaybeSelfRef::default();
    let mut x = unsafe { Pin::new_unchecked(&mut x) };
    x.as_mut().init();
    println!("{}", x.as_ref().a);
    *x.as_mut().b().unwrap() = 2;
    println!("{}", x.as_ref().a);
}

use std::mem::swap;
fn stack_pinning_manual_problem() {
    let mut x = MaybeSelfRef::default();
    let mut y = MaybeSelfRef::default();

    {
        let mut x = unsafe { Pin::new_unchecked(&mut x) };
        x.as_mut().init();
        *x.as_mut().b().unwrap() = 2;
    }
    swap(&mut x, &mut y);
    println!("
     x: {{
  +----->a: {:p},
  |      b: {:?},
  |  }}
  |
  |  y: {{
  |      a: {:p},
  +-----|b: {:?},
     }}",
        &x.a,
        x.b,
        &y.a,
        y.b,
    );
}

fn stack_pinning_macro() {
    let mut x = pin!(MaybeSelfRef::default());
    MaybeSelfRef::init(x.as_mut());
    println!("{}", x.as_ref().a);
    *x.as_mut().b().unwrap() = 2;
    println!("{}", x.as_ref().a);
}

fn pin_projection() {
    #[derive(Default)]
    struct Foo {
        a: MaybeSelfRef,
        b: String,
    }

    impl Foo {
        fn a(self: Pin<&mut Self>) -> Pin<&mut MaybeSelfRef> {
            unsafe {
                self.map_unchecked_mut(|s| &mut s.a)
            }
        }

        fn b(self: Pin<&mut Self>) -> &mut String {
            unsafe {
                &mut self.get_unchecked_mut().b
            }
        }
    }
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
