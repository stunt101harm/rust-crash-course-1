struct Person {
    name: String,
    age: u64,
    height: u64,
    weight: f64,
}

impl Person {
    fn print_name(&self) {
        println!("name: {:?}", self.name);
    }

    fn print_age(&self) {
        println!("age: {:?}", self.age);
    }
    fn print_height(&self) {
        println!("height (cm): {:?}", self.height);
    }
    fn print_weight(&self) {
        println!("weight (lbs): {:?}", self.weight);
    }

    fn print_details(&self) {
        println!(
            "name: {:?}, age: {:?}, height(cm): {:?}, weight(lbs): {:?}",
            self.name, self.age, self.height, self.weight
        )
    }
}

fn main() {
    let bob = Person {
        name: "bob".to_string(),
        age: 25,
        height: 175,
        weight: 150.5,
    };
    bob.print_details();
    bob.print_age();
    bob.print_height();
}
