use d08_vec_hashmap::*;

#[test]
#[ignore]
fn grouping() {
    let groups = group_by_len(&["a", "bb", "c", "dd", "eee"]);
    assert_eq!(groups[&1usize], vec!["a", "c"]);
    assert_eq!(groups[&2usize], vec!["bb", "dd"]);
    assert_eq!(groups[&3usize], vec!["eee"]);
    assert_eq!(groups.len(), 3);
}
