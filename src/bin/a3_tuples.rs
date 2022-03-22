fn main() {
    let tuple: (u8, bool, f32, String) = (5, true, 5.5, "hello".to_owned());
    let second_tuple = (5, 10);

    // print structure of the array and other objects

    println!(
        "first: {}, second: {}, third: {}, fourth: {}",
        tuple.0, tuple.1, tuple.2, tuple.3
    );
    println!("{:?}", second_tuple);

    // destructuring

    let (a, b, c, d) = tuple;
    println!("first: {}, second: {}, third: {}, fourth: {}", a, b, c, d);
}
