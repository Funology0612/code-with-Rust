struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn main() {
    let mocha = make_coffee(String::from("Mocha"), 4.99, true);

    let caramel_macchiato = Coffee {
        name: mocha.name.clone(),
        ..mocha
    };

    println!("{}", caramel_macchiato.name);
    println!("{}", mocha.name);
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}
