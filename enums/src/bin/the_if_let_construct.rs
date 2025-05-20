enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::Whole;

    if let Milk::Whole = my_beverage {
        println!("You have the Whole milk");
    }

    let my_beverage = Milk::Lowfat(3);

    if let Milk::Lowfat(percent) = my_beverage {
        println!("Your beverage is {percent}% milk.");
    }

    let my_beverage = Milk::NonDairy {
        kind: String::from("oat"),
    };

    if let Milk::NonDairy { kind } = my_beverage {
        println!("{kind} milk");
    }
}
