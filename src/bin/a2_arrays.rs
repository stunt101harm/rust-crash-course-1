fn main() {
    let array: [u8; 3] = [1, 2, 3];
    let second_array: [u8; 5] = [100; 5];

    println!(
        "index: {}, length of second array: {}",
        array[0],
        second_array.len()
    );

    // print structure of the array and other objects
    // expected: [100, 100, 100, 100, 100]

    println!("{:?}", second_array);
}
