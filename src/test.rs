use pretty_assertions::assert_eq;
use std::fs::read;

use crate::{
    find::{Finder, Finding},
    remove_slice,
};

#[test]
fn test_find() {
    let mut finder = Finder::new().unwrap();

    let buf = "dbg!(\"hello\");\n\ndbg!(1);\n".as_bytes();

    assert_eq!(
        finder.find(buf),
        vec![Finding::new(0, 13), Finding::new(16, 7)]
    );
}

const REPLACE_TESTS: &[&str] = &["hello", "double", "expr", "nested"];

#[test]
fn test_replacements() {
    let mut finder = Finder::new().unwrap();
    for test in REPLACE_TESTS {
        let in_file = format!("fixtures/{test}.in.rs");
        let out_file = format!("fixtures/{test}.out.rs");

        let buf = read(in_file).unwrap();

        let actual = remove_slice(&mut finder, buf).unwrap();

        let expected = read(out_file).unwrap();

        assert_eq!(
            String::from_utf8(actual).unwrap(),
            String::from_utf8(expected).unwrap()
        );
    }
}
