use d02_ownership::*;

#[test]
#[ignore]
fn drain_evens_works() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let evens = drain_evens(&mut v);
    assert_eq!(evens, vec![2, 4, 6]);
    assert_eq!(v, vec![1, 3, 5]);
}

#[test]
#[ignore]
fn drain_evens_no_evens() {
    let mut v = vec![1, 3];
    assert!(drain_evens(&mut v).is_empty());
    assert_eq!(v, vec![1, 3]);
}
