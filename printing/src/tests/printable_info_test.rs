use crate::PrintableInfo::*;

#[test]
fn display_info_removed() {
    let info = Removed {
        desc: "foo".to_string(),
    };
    assert_eq!(
        format!("{}", info),
        "\u{1b}[1;2;37minfo\u{1b}[0m: Removed \"foo\""
    );
}
