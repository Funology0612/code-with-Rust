fn main() {
    println!("{}", "Coke" == "Pepsi");
    println!("{}", "Coke" != "Pepsi");
    println!("{}", "Coke" == "coke");
    println!("{}", "Coke" == "Coke ");
    println!("{}", "Coke" == "Coke");

    println!("{}", 13 == 13);
    println!("{}", 13 != 13);

    println!("{}", 26.1 == 26.1);
    println!("{}", 26.1 == 26.14);

    println!("{}", 13 == 13.1 as i32);

    println!("{}", true == true);
    println!("{}", false == false);
    println!("{}", true != false);
}
