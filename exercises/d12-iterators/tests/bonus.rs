use d12_iterators::*;

#[test]
#[ignore]
fn argmax_works() {
    assert_eq!(argmax(&[0.1, 0.9, 0.3]), Some(1));
    assert_eq!(argmax(&[-3.0, -1.0, -2.0]), Some(1));
    assert_eq!(argmax(&[]), None);
}

#[test]
#[ignore]
fn softmax_is_a_distribution() {
    let p = softmax(&[1.0, 2.0, 3.0]);
    let sum: f32 = p.iter().sum();
    assert!((sum - 1.0).abs() < 1e-6);
    assert!(p[0] < p[1] && p[1] < p[2]);
}

#[test]
#[ignore]
fn softmax_is_stable_with_large_inputs() {
    let p = softmax(&[1000.0, 1000.0]);
    assert!(p.iter().all(|x| x.is_finite()));
    assert!((p[0] - 0.5).abs() < 1e-6);
}
