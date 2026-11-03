use d01_variables_types::*;

#[test]
#[ignore]
fn average() {
    assert_eq!(checked_average(&[1, 2, 3, 4]), Some(2.5));
    assert_eq!(checked_average(&[]), None);
}

#[test]
#[ignore]
fn fizzbuzz_list() {
    assert_eq!(fizzbuzz_up_to(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    assert!(fizzbuzz_up_to(0).is_empty());
}
