use d09_strings::*;

#[test]
fn palindromes() {
    assert!(is_palindrome("A man, a plan, a canal: Panama"));
    assert!(is_palindrome("kayak"));
    assert!(is_palindrome(""));
    assert!(!is_palindrome("hello"));
}

#[test]
fn vowels() {
    assert_eq!(count_vowels("Hello World"), 3);
    assert_eq!(count_vowels("AEIOU"), 5);
    assert_eq!(count_vowels("rhythm"), 0);
}

#[test]
fn caesar_basic() {
    assert_eq!(caesar("abc xyz", 3), "def abc");
    assert_eq!(caesar("Hello, World!", 13), "Uryyb, Jbeyq!");
}

#[test]
fn caesar_wraps_and_big_shift() {
    assert_eq!(caesar("abc", 26), "abc");
    assert_eq!(caesar("abc", 255), caesar("abc", 255 % 26));
}
