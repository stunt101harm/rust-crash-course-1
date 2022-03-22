fn main() {
    let array = [0, 1, 2, 3, 4];
    let slice = &array[1..3];

    fn borrow_slice(array: [i32; 5], slice: &[i32]) {
        println!("{:?}", array);
        println!("{:?}", slice);
        println!("length: {}", slice.len());
        println!("{} {}", slice[0], slice[1]);
    }
    borrow_slice(array, slice);
}
