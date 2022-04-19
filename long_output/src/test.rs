use std::io::Write;

use crate::*;

#[test]
fn prints_to_primary_for_short_output() {
    let mut primary = Vec::new();
    let mut alternate = Vec::new();
    {
        let mut out = max_lines(10)
            .primary(&mut primary)
            .alternate(|| &mut alternate);
        out.write_all("a\n".as_bytes()).unwrap();
        out.write_all("b\n".as_bytes()).unwrap();
        out.flush().unwrap();
    }
    assert_eq!(primary, "a\nb\n".as_bytes());
    assert_eq!(alternate, "".as_bytes());
}

#[test]
fn prints_to_secondary_for_long_output() {
    let mut primary = Vec::new();
    let mut alternate = Vec::new();
    {
        let mut out = max_lines(4)
            .primary(&mut primary)
            .alternate(|| &mut alternate);
        out.write_all("a\n".as_bytes()).unwrap();
        out.write_all("b\n".as_bytes()).unwrap();
        out.write_all("c\n".as_bytes()).unwrap();
        out.write_all("d\n".as_bytes()).unwrap();
        out.write_all("e\n".as_bytes()).unwrap();
    }
    assert_eq!(primary, "".as_bytes());
    assert_eq!(alternate, "a\nb\nc\nd\ne\n".as_bytes());
}
