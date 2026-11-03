use d14_mini_matrix::*;

fn m(rows: usize, cols: usize, data: &[f32]) -> Matrix {
    Matrix::from_vec(rows, cols, data.to_vec()).unwrap()
}

#[test]
fn zeros_and_dims() {
    let z = Matrix::zeros(2, 3);
    assert_eq!(z.rows(), 2);
    assert_eq!(z.cols(), 3);
    assert_eq!(z.get(1, 2), 0.0);
}

#[test]
fn from_vec_checks_size() {
    assert!(Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0]).is_err());
    assert!(Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]).is_ok());
}

#[test]
fn get_set_and_row_major_layout() {
    let mut a = m(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    assert_eq!(a.get(0, 2), 3.0);
    assert_eq!(a.get(1, 0), 4.0);
    a.set(1, 1, 50.0);
    assert_eq!(a.get(1, 1), 50.0);
    assert_eq!(a.row(0), &[1.0, 2.0, 3.0]);
    assert_eq!(a.row(1), &[4.0, 50.0, 6.0]);
}

#[test]
fn transpose_works() {
    let a = m(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let t = a.transpose();
    assert_eq!(t, m(3, 2, &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]));
    assert_eq!(t.transpose(), a);
}

#[test]
fn matmul_square() {
    let a = m(2, 2, &[1.0, 2.0, 3.0, 4.0]);
    let b = m(2, 2, &[5.0, 6.0, 7.0, 8.0]);
    assert_eq!(a.matmul(&b), Some(m(2, 2, &[19.0, 22.0, 43.0, 50.0])));
}

#[test]
fn matmul_rectangular() {
    let a = m(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let b = m(3, 2, &[7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
    assert_eq!(a.matmul(&b), Some(m(2, 2, &[58.0, 64.0, 139.0, 154.0])));
}

#[test]
fn matmul_incompatible() {
    let a = m(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    assert_eq!(a.matmul(&a), None);
}
