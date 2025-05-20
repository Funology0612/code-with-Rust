enum Cheesesteak<T> {
    Plain,
    Topping(T),
}

fn main() {
    let mushroom = Cheesesteak::Topping("mushroom");
    let onions = Cheesesteak::Topping("onions".to_string());
    let topping = "bacon".to_string();
    let bacon = Cheesesteak::Topping(&topping);

    let mut plain: Cheesesteak<String> = Cheesesteak::Plain;

    // Invalid, &str is not a String, which is what T must be for plain variable
    // plain = Cheesesteak::Topping("sausage");
}
