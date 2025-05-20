fn main() {
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = musical_instruments.get(2);
    println!("{:?}", bass);
    let valid_instrument = bass.expect("Unable to retrieve element");
    println!("{valid_instrument}");

    let invalid_instrument = musical_instruments.get(100);
    println!("{:?}", invalid_instrument);
    invalid_instrument.expect("Unable to retrieve musical instrument");
}
