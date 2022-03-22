fn main() {
    let mut i = 5;
    match i {
        0 => println!("0"),
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        _ => println!("not in range"),
    }
}
