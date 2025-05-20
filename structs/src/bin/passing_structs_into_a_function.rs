struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}

fn main() {
    let mut mocha = make_coffee(String::from("Mocha"), 4.99, true);
    drink_coffee(&mut mocha);

    println!("{}", mocha.price);
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}

fn drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my delicious {}", coffee.name);
    coffee.is_hot = false;
    coffee.price = 10.99;
}
