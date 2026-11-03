use d04_structs::*;

#[test]
#[ignore]
fn square_detection() {
    assert!(Rectangle::square(3.0).is_square());
    assert!(!Rectangle::new(3.0, 4.0).is_square());
}

#[test]
#[ignore]
fn largest_by_area() {
    let rects = vec![
        Rectangle::new(1.0, 1.0),
        Rectangle::new(5.0, 2.0),
        Rectangle::new(3.0, 3.0),
    ];
    assert_eq!(largest(&rects), Some(&Rectangle::new(5.0, 2.0)));
    assert_eq!(largest(&[]), None);
}
