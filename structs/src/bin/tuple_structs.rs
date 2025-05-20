// Hours, minutes
struct ShortDuration(i32, i32);
// Years, months
struct LongDuration(i32, i32);

fn main() {
    let work_shift = ShortDuration(8, 30);
    println!("{} hours {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} years {} months", era.0, era.1);

    // go_to_work(era);
    // accept_tuple(era);

    // let work_shift = (8, 0);
    // let era = (5, 3);
    // go_to_work(work_shift);
    // go_to_work(era);
}

fn go_to_work(length: ShortDuration) {
    println!("Passing time {} hours {} minutes", length.0, length.1);
}

fn accept_tuple(length: (u32, u32)) {}
