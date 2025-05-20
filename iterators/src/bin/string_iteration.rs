fn main() {
    let seafood = String::from("Oyster🦪");

    for byte in seafood.bytes() {
        print!("{byte}/");
    }

    println!();

    for character in seafood.chars() {
        print!("{character}/");
    }

    println!("{seafood}");

    println!("{}", seafood.bytes().len()); // 10
    println!("{}", seafood.chars().count()); // 7
}
