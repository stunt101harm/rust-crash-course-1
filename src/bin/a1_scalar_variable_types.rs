fn main() {
    // unsigned integers (only positive integers)
    // u8, u16, u32, u64, u128

    let unsigned_integer: u8 = 10;
    let unsigned_integers = 5;

    // signed integers (can be positive or negative)
    // i8, i16, i32, i64, i128

    let signed_integer: i8 = -10;
    let signed_integers = -1;

    // float type is used for decimals

    let float: f32 = 25.5;
    let floats = 10.1;

    println!(
        "unsigned integer: {}, signed integer: {}, float: {}",
        unsigned_integer, signed_integer, float
    );

    // string

    let string: String = "name".to_string();
    let new_string = String::from("new");

    println!("{}, {}", string, new_string);

    // booleans

    let is_true: bool = true;

    println!("is true: {}", is_true);
    
}
