use std::fmt::{Display, Formatter, Result};

struct Apple {
    kind: String,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{} 🍏 for {}", self.kind, self.price)
    }
}

fn main() {
    let lunch_snack = Apple {
        kind: String::from("Granny Smith"),
        price: 1.04,
    };

    println!("{}", lunch_snack);
}
