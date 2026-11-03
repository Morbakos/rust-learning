use d11_generics::*;

#[test]
fn starts_empty() {
    let s: Stack<i32> = Stack::new();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
    assert_eq!(s.peek(), None);
}

#[test]
fn lifo_order() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.len(), 3);
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
    assert!(s.is_empty());
}

#[test]
fn peek_does_not_remove() {
    let mut s = Stack::new();
    s.push("a".to_string());
    s.push("b".to_string());
    assert_eq!(s.peek(), Some(&"b".to_string()));
    assert_eq!(s.len(), 2);
}
