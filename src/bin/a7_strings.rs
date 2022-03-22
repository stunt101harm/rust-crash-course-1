fn main() {
    let str: &str = "Hello";
    let mut string: String = String::from("hello world");
    let string2 = "hello".to_string();
    let string3 = "hello world".to_owned();

    let slice = &string[..6];
    slice.len();
    println!("{}", slice.len());

    string.push('_');
    string.push_str("hello world");
    println!("{}", string);
    string = string.replace("hello world", "good night");
    println!("{}", string);
}
