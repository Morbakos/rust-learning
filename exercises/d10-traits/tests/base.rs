use d10_traits::*;

struct Duck;

impl Animal for Duck {
    fn name(&self) -> String {
        "Duck".to_string()
    }
    fn sound(&self) -> String {
        "Quack".to_string()
    }
}

#[test]
fn dog_and_cat() {
    assert_eq!(Dog.name(), "Dog");
    assert_eq!(Dog.sound(), "Woof");
    assert_eq!(Cat { lives: 9 }.name(), "Cat");
    assert_eq!(Cat { lives: 9 }.sound(), "Meow");
}

#[test]
fn default_method_is_shared() {
    assert_eq!(Dog.describe(), "Dog says Woof");
    assert_eq!(Duck.describe(), "Duck says Quack");
}

#[test]
fn heterogeneous_list() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat { lives: 9 }),
        Box::new(Duck),
    ];
    assert_eq!(
        describe_all(&animals),
        vec!["Dog says Woof", "Cat says Meow", "Duck says Quack"]
    );
}

#[test]
fn largest_generic() {
    assert_eq!(largest(&[3, 7, 2]), Some(7));
    assert_eq!(largest(&[1.5, 0.5]), Some(1.5));
    assert_eq!(largest(&['a', 'z', 'c']), Some('z'));
    assert_eq!(largest::<i32>(&[]), None);
}
