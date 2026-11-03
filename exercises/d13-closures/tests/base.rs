use d13_closures::*;

#[test]
fn apply_n_times() {
    assert_eq!(apply_n(|x| x * 2, 3, 1), 8);
    assert_eq!(apply_n(|x| x + 10, 0, 5), 5);
}

#[test]
fn adder() {
    let add5 = make_adder(5);
    assert_eq!(add5(10), 15);
    assert_eq!(add5(-5), 0);
}

#[test]
fn sort_by_length_is_stable() {
    let mut words: Vec<String> = ["ccc", "a", "bb"].iter().map(|s| s.to_string()).collect();
    sort_by_len(&mut words);
    assert_eq!(words, vec!["a", "bb", "ccc"]);

    let mut tie: Vec<String> = ["bb", "aa", "c"].iter().map(|s| s.to_string()).collect();
    sort_by_len(&mut tie);
    assert_eq!(tie, vec!["c", "bb", "aa"]);
}

#[test]
fn counting_closure() {
    let mut c = counter();
    assert_eq!(c(), 1);
    assert_eq!(c(), 2);
    assert_eq!(c(), 3);
    let mut other = counter();
    assert_eq!(other(), 1);
}
