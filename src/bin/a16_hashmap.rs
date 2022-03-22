use std::collections::HashMap;

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
}
