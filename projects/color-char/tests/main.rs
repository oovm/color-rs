use color_char::Character;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn test_bits21() {
    let mut c = Character::from(0b000011010_000000000000001011010);
    c.set_char('Z');
    c.set_color(26);
    println!("{:#?}", c);
    println!("{:#b}", c);
}

#[test]
pub fn test_get() {
    let c = Character::from(0b000011010_000000000000001011010);
    assert_eq!(c.get_char(), 'Z');
    assert_eq!(c.get_color(), 26);
}

#[test]
pub fn test_set() {
    let mut c = Character::default();
    c.set_char('Z');
    assert_eq!(c, 'Z');
}
