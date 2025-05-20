fn main() {
    let result = square(5);
    println!("The square of 5 is {result}");

    let result = square(13);
    println!("The square of 13 is {result}");
}

fn square(number: i32) -> i32 {
    number * number
}
