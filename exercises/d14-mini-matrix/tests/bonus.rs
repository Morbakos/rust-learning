use d14_mini_matrix::*;

#[test]
#[ignore]
fn matvec_works() {
    let a = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
    assert_eq!(a.matvec(&[1.0, 0.0, -1.0]), Some(vec![-2.0, -2.0]));
    assert_eq!(a.matvec(&[1.0, 2.0]), None);
}
