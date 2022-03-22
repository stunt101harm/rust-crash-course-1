fn main() {
    let mut vector = vec![0, 1, 2, 3, 4];
    println!("{:?}", vector.len());
    vector[0];
    vector.push(5);
    vector.remove(1);
    vector.pop();
    vector.is_empty();
    println!("{:?}", vector);
}
