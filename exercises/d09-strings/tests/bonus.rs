use d09_strings::*;

#[test]
#[ignore]
fn chars_vs_bytes() {
    assert_eq!(char_count_vs_byte_len("abc"), (3, 3));
    assert_eq!(char_count_vs_byte_len("héé"), (3, 5));
}

#[test]
#[ignore]
fn truncate() {
    assert_eq!(truncate_chars("héllo", 2), "hé");
    assert_eq!(truncate_chars("abc", 10), "abc");
    assert_eq!(truncate_chars("abc", 0), "");
}
