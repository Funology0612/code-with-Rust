fn main() {
    let values = [4, 8, 15, 16, 23, 42];

    let regular_reference = &values;
    print_length(regular_reference);

    let slice_of_three = &values[0..3];
    print_length(slice_of_three);
}

fn print_length(slice: &[i32]) {
    println!("{}", slice.len());
}
