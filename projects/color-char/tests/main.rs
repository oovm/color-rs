#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn test_bits21() {
    assert_eq!('a' as u32, 0b000000000000000000000001100001);
    let mut c = Character::from(0b_00000001_001_0000_00111111_11111111);
    c.set_char('a');
    c.set_color(2047);
    println!("{:#?}", c);
    println!("{:#b}", c);
}
