fn main() {
    let pizza_diameters = vec![8, 10, 12, 14];

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];

    let pizza_slice = &pizza_toppings[1..];
    println!("{pizza_slice:?}");
}
