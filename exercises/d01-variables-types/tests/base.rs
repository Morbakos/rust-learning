use d01_variables_types::*;

#[test]
fn celsius_to_fahrenheit_works() {
    assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    assert!((celsius_to_fahrenheit(-40.0) + 40.0).abs() < 1e-9);
}

#[test]
fn fizzbuzz_cases() {
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(15), "FizzBuzz");
    assert_eq!(fizzbuzz(7), "7");
}

#[test]
fn min_max_basic() {
    assert_eq!(min_max(&[3, -1, 8, 2]), Some((-1, 8)));
    assert_eq!(min_max(&[5]), Some((5, 5)));
}

#[test]
fn min_max_empty() {
    assert_eq!(min_max(&[]), None);
}
