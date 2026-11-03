use d05_enums_match::*;

#[test]
fn areas() {
    assert_eq!(area(&Shape::Rect { width: 2.0, height: 3.0 }), 6.0);
    assert_eq!(area(&Shape::Triangle { base: 4.0, height: 3.0 }), 6.0);
    let circle = area(&Shape::Circle { radius: 1.0 });
    assert!((circle - std::f64::consts::PI).abs() < 1e-9);
}

#[test]
fn parse_valid_commands() {
    assert_eq!(parse_command("quit"), Some(Command::Quit));
    assert_eq!(
        parse_command("move 3 -2"),
        Some(Command::Move { dx: 3, dy: -2 })
    );
    assert_eq!(
        parse_command("say hello world"),
        Some(Command::Say("hello world".to_string()))
    );
}

#[test]
fn parse_invalid_commands() {
    assert_eq!(parse_command("jump"), None);
    assert_eq!(parse_command("move 3"), None);
    assert_eq!(parse_command("move a b"), None);
    assert_eq!(parse_command(""), None);
}
