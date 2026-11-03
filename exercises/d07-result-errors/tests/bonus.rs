use d07_result_errors::*;

fn assert_is_error<E: std::error::Error>(_: &E) {}

#[test]
#[ignore]
fn display_messages() {
    let msg = ParseError::EmptyKey.to_string();
    assert!(!msg.is_empty());
    let msg = ParseError::InvalidValue("abc".to_string()).to_string();
    assert!(msg.contains("abc"));
    assert_is_error(&ParseError::MissingSeparator);
}
