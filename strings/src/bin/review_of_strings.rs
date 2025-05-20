fn main() {
    // &str
    // String

    let pirate = "Bloodhook";

    let sailer = String::from(pirate);

    let bad_guy = sailer.to_string();

    println!("{pirate} and {sailer} and {bad_guy}");

    let first_initial = &pirate[0..4];
    println!("{first_initial}");
}
