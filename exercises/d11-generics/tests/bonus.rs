use d11_generics::*;

#[test]
#[ignore]
fn render_stack() {
    let mut s: Stack<i32> = Stack::new();
    assert_eq!(s.render(), "[]");
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.render(), "[1, 2, 3]");
}

#[test]
#[ignore]
fn max_of_stack() {
    let mut s: Stack<i32> = Stack::new();
    assert_eq!(s.max(), None);
    s.push(3);
    s.push(9);
    s.push(4);
    assert_eq!(s.max(), Some(9));
}
