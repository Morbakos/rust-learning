use d04_structs::*;

#[test]
fn construction() {
    let r = Rectangle::new(3.0, 4.0);
    assert_eq!(r.width, 3.0);
    assert_eq!(r.height, 4.0);
    assert_eq!(Rectangle::square(2.0), Rectangle::new(2.0, 2.0));
}

#[test]
fn area_and_perimeter() {
    let r = Rectangle::new(3.0, 4.0);
    assert_eq!(r.area(), 12.0);
    assert_eq!(r.perimeter(), 14.0);
}

#[test]
fn can_hold_rules() {
    let big = Rectangle::new(10.0, 8.0);
    let small = Rectangle::new(5.0, 4.0);
    let wide = Rectangle::new(20.0, 2.0);
    assert!(big.can_hold(&small));
    assert!(!small.can_hold(&big));
    assert!(!big.can_hold(&wide));
    assert!(!big.can_hold(&big));
}

#[test]
fn scale_in_place() {
    let mut r = Rectangle::new(2.0, 3.0);
    r.scale(2.0);
    assert_eq!(r, Rectangle::new(4.0, 6.0));
}
