use d13_closures::*;

#[test]
#[ignore]
fn composition() {
    let f = compose(|x: i32| x + 1, |y: i32| y * 2);
    assert_eq!(f(3), 8);
    let g = compose(|s: &str| s.len(), |n: usize| n % 2 == 0);
    assert!(g("ab"));
    assert!(!g("abc"));
}
