fn main() {

}

struct SelfRef<'a> {
    numbers: [usize; 3],
    picked: Option<&'a usize>,
}

impl<'a> SelfRef<'a> {
    fn new() -> Self {
        SelfRef {numbers: [1,2,3], picked: None}
    }

    fn pick(&mut self, n: usize) {
        self.picked = Some(&self.numbers[n]);
    }

    fn show(&self) -> usize {
        *self.picked.unwrap()
    }
}
