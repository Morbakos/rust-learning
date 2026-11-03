use d10_traits::*;

#[test]
#[ignore]
fn cat_display() {
    assert_eq!(format!("{}", Cat { lives: 9 }), "Cat(9 lives)");
    assert_eq!(Cat { lives: 1 }.to_string(), "Cat(1 lives)");
}
