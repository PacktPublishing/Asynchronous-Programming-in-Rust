use std::{fs, env::temp_dir};

use corofy::rewrite;
#[test]
fn produces_expected_output() {
    let src = fs::read_to_string("./tests/input.txt").unwrap();
    let dest_path = temp_dir().join("test.txt");
    let dest = fs::File::create(&dest_path).unwrap();

    rewrite(src, dest);

    let expected = fs::read_to_string("./tests/expected.txt").unwrap();
    let got = fs::read_to_string(dest_path).unwrap();

    for (i, (a, b)) in got.lines().zip(expected.lines()).enumerate() {
        assert_eq!(a.trim(), b.trim(), "Failed in line {}", i+1);
    }
}