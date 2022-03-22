use std::collections::HashMap;

// Options takes either Some value or None
// returns either Ok or Err

fn divide(number: i32, divisor: i32) -> Result<i32, String> {
    if number % divisor != 0 {
        Err(("not divisble".to_string()))
    } else {
        Ok(number / divisor)
    }
}
fn main() {
    let mut hashmap = HashMap::new();
    hashmap.insert(0, "first");
    hashmap.insert(1, "second");
    println!("{:?}", hashmap);

    match hashmap.get(&0) {
        Some(str) => println!("{}", str),
        _ => println!("isn't in the hashmap"),
    }
    match hashmap.get(&4) {
        Some(str) => println!("{}", str),
        _ => println!("doesn't exist"),
    }
    hashmap.remove(&0);
    println!("{:?}", hashmap);

    let answer = divide(25, 5);
    let no_answer = divide(30, 20);
    println!("{:?}", answer);
    println!("{:?}", answer.unwrap()); // use unwrap to extract value wrapped in 'Some'
    println!("{:?}", no_answer); // Cannot unwrap a 'None' value
}
