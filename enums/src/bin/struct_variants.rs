#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal { username: String, password: String },
    Cash,
}

fn main() {
    let visa = PaymentMethodType::CreditCard(String::from("0012-3456"));

    let paypal = PaymentMethodType::PayPal {
        username: String::from("bob@gmail.com"),
        password: String::from("password"),
    };
    println!("{:?}", paypal);
}
