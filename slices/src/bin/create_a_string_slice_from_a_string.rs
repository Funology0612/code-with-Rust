fn main() {
    let action_hero = String::from("Arnold Schwarzenegger");
    let first_name = &action_hero[0..6];
    println!("{first_name}");

    let last_name = &action_hero[7..22];
    // println!("{last_name}");
}
