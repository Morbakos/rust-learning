use d02_ownership::*;

#[test]
fn give_back_with_len() {
    let (s, len) = take_and_give_back(String::from("héllo"));
    assert_eq!(s, "héllo");
    assert_eq!(len, 6); // 'é' occupe 2 octets en UTF-8
}

#[test]
fn exclaim() {
    let mut s = String::from("hi");
    add_exclaim(&mut s);
    assert_eq!(s, "hi!");
    add_exclaim(&mut s);
    assert_eq!(s, "hi!!");
}

#[test]
fn concat() {
    assert_eq!(concat_owned(String::from("ab"), "cd"), "abcd");
    assert_eq!(concat_owned(String::new(), "x"), "x");
}
