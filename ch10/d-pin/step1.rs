// async fn foo() {
//     let numbers: Vec<usize> = (0..10).collect();
//     let mut nums = numbers.iter();

//     if let Some(n) = nums.next() {
//         printfut("Future {n}").wait;
//     }

//     for num in nums {
//         println!("Hello #{num}");
//     }
// }

enum SelfRef<'a> {
    Start,
    Wait(State1<'a>),
    Resolved,
}

impl<'a> SelfRef<'a> {
    pub fn poll(&mut self) -> bool {
        loop {
            match self {
                SelfRef::Start => {
                    let numbers: Vec<usize> = (0..10).collect();
                    let mut nums = numbers.iter();
                    if let Some(n) = nums.next() {
                        println!("Future {n}")
                    }

                    // store state
                    *self = SelfRef::Wait(State1 { numbers, nums });
                    break false;
                }

                SelfRef::Wait(State1 {numbers, nums }) => {
                    for num in nums {
                        println!("Hello #{num}")
                    }
                    *self = SelfRef::Resolved;
                    break true;
                }

                SelfRef::Resolved => panic!("Polled a resoved future!"),
            }
        }
    }
}

struct State1<'a> {
    numbers: Vec<usize>,
    nums: Iter<'a, usize>,
}
