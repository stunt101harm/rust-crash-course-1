fn is_even(num: u8) -> bool {
    let number: u8 = num % 2;
    number == 0
}

fn is_positive(num: i32) -> bool {
    let integer = num > 0;
    integer == true
}

fn main() {
    println!("is even: {}", is_even(4));

    println!("is positive: {}", is_positive(-1));
}
