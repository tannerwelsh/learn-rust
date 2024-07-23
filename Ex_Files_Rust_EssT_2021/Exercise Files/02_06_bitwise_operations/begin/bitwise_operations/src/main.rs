fn main() {
    let mut value = 0b11110101;
    println!("value is {:08b}", value);
    value = !value;
    println!("value is {:08b}", value);
}
