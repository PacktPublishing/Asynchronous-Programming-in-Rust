// fn main() {
//     let list1: Vec<SelfRef> = (0..10).map(|_| {
//         let mut picker = SelfRef::new();
//         picker.pick(2);
//         picker
//     }).collect();

//     list1.iter().for_each(|p| println!("{}", p.show()));

//     let list2 = Vec::from_iter(list1.iter());
//     list2.iter().for_each(|p| println!("{}", p.show()));
// }
fn main() {
    let mut picker1 = Picker::new();
    picker1.pick(2);
    println!("{}", picker1.show());
    let replaced = std::mem::replace(&mut picker1, Picker::new());
    println!("{}", replaced.show());
}

#[derive(Clone, Default)]
struct Picker {
    numbers: Option<[usize; 3]>,
    picked: Option<*const usize>,
}

impl Picker {
    fn new() -> Self {
        Picker {
            numbers: None,
            picked: None,
        }
    }

    fn pick(&mut self, n: usize) {
        self.numbers = Some([1, 2, 3]);
        self.picked = Some(&self.numbers.as_ref().unwrap()[n]);
    }

    fn show(&self) -> String {
        match self.picked {
            Some(p) => format!("Pick: {}", unsafe {*p}),
            None => format!("Nothing picked yet"),
        }
    }
}

// fn main() {
//     let code_table = ['d','h','e','x','l','w','p','y','z'];

//     let mut selfref = SelfRef {
//         code_table,
//         encoder: Encoder::new(),
//     };

//     selfref.encoder.set(&selfref.code_table);

//     selfref.encoder.encode(&[1,2,3,2,3,4,5,2]);
//     println!("{}", selfref.encoder.txt);
// }

// struct SelfRef<'a> {
//     code_table: [char; 9],
//     encoder: Encoder<'a>,
// }

// struct Encoder<'a> {
//     code_table: Option<&'a [char; 9]>,
//     txt: String,
// }

// impl<'a> Encoder<'a> {
//     fn new() -> Self {
//         Encoder { code_table: None, txt: String::new() }
//     }
//     fn set(&mut self, code_table: &'a [char; 9]) {
//         self.code_table = Some(code_table);
//     }
//     fn encode(&mut self, numbers: &[usize]) {
//         let table = self.code_table.expect("Not set");
//         for num in numbers {
//             self.txt.push(table[*num]);
//         }
//     }
// }

// struct SecretFmt<'a> {
//     random: &'a String,
//     something_big: Vec<u8>,
// }

// impl<'a> SecretFmt<'a> {
//     fn new(random: &'a String) -> Self {
//         Self {
//             random,
//             something_big: vec![0u8; 1024 * 1000 * 1000],
//         }
//     }

//     fn fmt(&self, txt: &str) -> String {
//         format!("{}: {}", self.random, txt)
//     }
// }

// struct MyFormatter<'a> {
//     data: String,
//     formatter: SecretFmt<'a>,
// }
