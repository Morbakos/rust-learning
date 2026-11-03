use d03_borrowing_slices::*;

#[test]
#[ignore]
fn halves() {
    let data = [1, 2, 3, 4, 5];
    let (left, right) = split_halves(&data);
    assert_eq!(left, &[1, 2]);
    assert_eq!(right, &[3, 4, 5]);
}

#[test]
#[ignore]
fn window_sum() {
    let data = [1, 2, 3, 4, 5];
    assert_eq!(largest_window_sum(&data, 2), Some(9));
    assert_eq!(largest_window_sum(&data, 5), Some(15));
    assert_eq!(largest_window_sum(&data, 6), None);
    assert_eq!(largest_window_sum(&data, 0), None);
}
