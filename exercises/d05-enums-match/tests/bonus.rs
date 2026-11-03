use d05_enums_match::*;

#[test]
#[ignore]
fn apply_commands() {
    assert_eq!(apply((1, 1), &Command::Move { dx: 2, dy: -3 }), (3, -2));
    assert_eq!(apply((1, 1), &Command::Quit), (1, 1));
    assert_eq!(apply((1, 1), &Command::Say("hi".to_string())), (1, 1));
}
