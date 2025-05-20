fn main() {
    let action_hero = String::from("Arnold Schwarzenegger");

    let first_name = &action_hero[..6];
    println!("His first name is {first_name}.");

    let last_name = &action_hero[7..];
    println!("His first name is {last_name}.");

    let full_name = &action_hero[..];
    println!("His full name is {full_name}.");
}
