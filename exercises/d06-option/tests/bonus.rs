use d06_option::*;

#[test]
#[ignore]
fn parse_halve() {
    assert_eq!(parse_and_halve("10"), Some(5));
    assert_eq!(parse_and_halve("7"), Some(3));
    assert_eq!(parse_and_halve("abc"), None);
}

#[test]
#[ignore]
fn upper() {
    assert_eq!(first_char_upper("rust"), Some('R'));
    assert_eq!(first_char_upper(""), None);
}
