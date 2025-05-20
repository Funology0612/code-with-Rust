fn main() {
    open_store("Brooklyn");
    bake_pizza(20, "pepperoni");
    swim_in_profit();
    swim_in_profit();
    swim_in_profit();
    open_store("Queens");
    bake_pizza(15, "mushroom");
}

fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {neighborhood}");
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizzas");
}

fn swim_in_profit() {
    println!("So much $$$, so little time");
}
