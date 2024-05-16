use crate::truncate::truncate;

#[test]
fn truncate_empty_list() {
    let actual = truncate(20, "...", &[]);
    let expected: Vec<&str> = vec![];
    assert_eq!(actual, expected);
}

#[test]
fn single_word_fits() {
    let actual = truncate(20, "...", &["word"]);
    let expected = vec!["word"];
    assert_eq!(actual, expected);
}

#[test]
fn two_words_fit() {
    let actual = truncate(20, "...", &["foo", "bar"]);
    let expected = vec!["foo", "bar"];
    assert_eq!(actual, expected);
}

#[test]
fn truncate_two_words_with_separator() {
    let actual = truncate(12, "...", &["foo", "bar", "baz", "qux"]);
    let expected = vec!["foo", "...", "qux"];
    assert_eq!(actual, expected);
}

#[test]
fn truncate_three_words_with_separator() {
    let actual = truncate(12, "...", &["foo", "bar", "baz", "qux", "quux"]);
    let expected = vec!["foo", "...", "quux"];
    assert_eq!(actual, expected);
}

#[test]
fn fits_within_max_width() {
    let actual = truncate(20, "...", &["foo", "bar", "baz"]);
    let expected = vec!["foo", "bar", "baz"];
    assert_eq!(actual, expected);
}

#[test]
fn right_end_does_not_fit() {
    let actual = truncate(12, "...", &["foo", "bar", "abcdefghijkl"]);
    let expected = vec!["foo", "bar", "..."];
    assert_eq!(actual, expected);
}

#[test]
fn left_end_does_not_fit() {
    let actual = truncate(12, "...", &["abcdefghijkl", "bar", "baz"]);
    let expected = vec!["...", "bar", "baz"];
    assert_eq!(actual, expected);
}

#[test]
fn truncation_separator_larger_than_max_width() {
    let actual = truncate(2, "...", &["foo", "bar", "baz"]);
    let expected: Vec<&str> = vec![];
    assert_eq!(actual, expected);
}
