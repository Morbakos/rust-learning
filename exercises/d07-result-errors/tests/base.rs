use d07_result_errors::*;

#[test]
fn parse_ok() {
    assert_eq!(parse_kv("age=42"), Ok(("age".to_string(), 42)));
    assert_eq!(parse_kv("t=-7"), Ok(("t".to_string(), -7)));
}

#[test]
fn parse_errors() {
    assert_eq!(parse_kv("novalue"), Err(ParseError::MissingSeparator));
    assert_eq!(parse_kv("=5"), Err(ParseError::EmptyKey));
    assert_eq!(
        parse_kv("x=abc"),
        Err(ParseError::InvalidValue("abc".to_string()))
    );
}

#[test]
fn sum_ok() {
    assert_eq!(sum_values(&["a=1", "b=2", "c=3"]), Ok(6));
    assert_eq!(sum_values(&[]), Ok(0));
}

#[test]
fn sum_stops_at_first_error() {
    assert_eq!(
        sum_values(&["a=1", "b", "=2"]),
        Err(ParseError::MissingSeparator)
    );
    assert_eq!(sum_values(&["a=1", "=2", "b"]), Err(ParseError::EmptyKey));
}
