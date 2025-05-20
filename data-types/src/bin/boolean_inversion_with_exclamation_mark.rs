fn main() {
    println!("{}", !true);
    println!("{}", !false);

    let age = 13;
    let can_see_rated_r_movie = age >= 17;
    let cannot_see_rated_r_movie = !can_see_rated_r_movie;
    println!("I am {age} years old. Can I not see this scary movie? {cannot_see_rated_r_movie}");
}
