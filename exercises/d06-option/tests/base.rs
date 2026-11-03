use d06_option::*;

#[test]
fn find() {
    assert_eq!(find_index(&[4, 5, 6, 5], 5), Some(1));
    assert_eq!(find_index(&[4, 5, 6], 9), None);
    assert_eq!(find_index(&[], 1), None);
}

#[test]
fn division() {
    assert_eq!(safe_div(10, 2), Some(5));
    assert_eq!(safe_div(7, 2), Some(3));
    assert_eq!(safe_div(1, 0), None);
    assert_eq!(safe_div(i32::MIN, -1), None);
}

#[test]
fn evens() {
    assert_eq!(first_even(&[1, 3, 4, 6]), Some(4));
    assert_eq!(first_even(&[1, 3]), None);
    assert_eq!(double_first_even(&[1, 3, 4, 6]), Some(8));
    assert_eq!(double_first_even(&[1, 3]), None);
}
