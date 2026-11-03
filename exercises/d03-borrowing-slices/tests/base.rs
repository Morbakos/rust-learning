use d03_borrowing_slices::*;

#[test]
fn first_word_cases() {
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(first_word("single"), "single");
    assert_eq!(first_word(""), "");
}

#[test]
fn sum() {
    assert_eq!(sum_slice(&[1, 2, 3, 4]), 10);
    assert_eq!(sum_slice(&[]), 0);
}

#[test]
fn reverse_even_and_odd() {
    let mut a = [1, 2, 3, 4];
    reverse_in_place(&mut a);
    assert_eq!(a, [4, 3, 2, 1]);

    let mut b = [1, 2, 3];
    reverse_in_place(&mut b);
    assert_eq!(b, [3, 2, 1]);
}

#[test]
fn reverse_part_of_a_vec() {
    let mut v = vec![1, 2, 3, 4, 5];
    reverse_in_place(&mut v[1..4]);
    assert_eq!(v, vec![1, 4, 3, 2, 5]);
}
