fn main() {
    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");

    let full_name = first_name + &last_name;
    println!("{full_name}");

    // Invalid
    // println!("{first_name}");
}
