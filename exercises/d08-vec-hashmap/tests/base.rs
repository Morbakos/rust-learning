use d08_vec_hashmap::*;

#[test]
fn counting() {
    let counts = word_count("a B a b A c");
    assert_eq!(counts.len(), 3);
    assert_eq!(counts["a"], 3);
    assert_eq!(counts["b"], 2);
    assert_eq!(counts["c"], 1);
}

#[test]
fn counting_empty() {
    assert!(word_count("").is_empty());
}

#[test]
fn top_with_ties() {
    let counts = word_count("b b a a c");
    assert_eq!(
        top_n(&counts, 2),
        vec![("a".to_string(), 2), ("b".to_string(), 2)]
    );
    assert_eq!(top_n(&counts, 10).len(), 3);
}

#[test]
fn dedup() {
    assert_eq!(dedup_sorted(&[3, 1, 3, 2, 1]), vec![1, 2, 3]);
    assert_eq!(dedup_sorted(&[]), Vec::<i32>::new());
}
