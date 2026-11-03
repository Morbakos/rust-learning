use d12_iterators::*;

#[test]
fn squares_of_evens() {
    assert_eq!(sum_of_squares_of_evens(&[1, 2, 3, 4]), 20);
    assert_eq!(sum_of_squares_of_evens(&[1, 3]), 0);
    assert_eq!(sum_of_squares_of_evens(&[]), 0);
}

#[test]
fn flatten() {
    assert_eq!(
        flatten_lines(&["Hello World", "", "Rust IS fun"]),
        vec!["hello", "world", "rust", "is", "fun"]
    );
}

#[test]
fn dot_product() {
    assert_eq!(dot(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]), 32.0);
    assert_eq!(dot(&[], &[]), 0.0);
}

#[test]
#[should_panic]
fn dot_length_mismatch() {
    dot(&[1.0, 2.0], &[1.0]);
}
