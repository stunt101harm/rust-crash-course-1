fn main() {
    let mut i = 3;
    while i < 10 {
        println!("{}", i);
        i += 1;
        if i == 9 {
            println!("done");
            break;
        }
    }
}
