fn main() {
    let availability = is_item_in_stock(true, false);

    match availability {
        Some(true) => println!("Yes, the item is available"),
        Some(false) => println!("No, the item is not in stock"),
        None => println!("Your item doesn't exist in our system"),
    }
}

fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Some(true)
    } else if item_is_in_system {
        Some(false)
    } else {
        None
    }
}
