fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];
    let print_numbers = || println!("{:?}", numbers);

    println!("{:?}", numbers);
    print_numbers();
    print_numbers();
    print_numbers();
    println!("{:?}", numbers);
}
