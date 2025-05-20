fn main() {
    let user_has_paid_for_subscription = false;
    let user_is_admin = false;
    let user_can_see_premium_experience = user_has_paid_for_subscription || user_is_admin;
    println!("Can this user see my site? {user_can_see_premium_experience}");
}
