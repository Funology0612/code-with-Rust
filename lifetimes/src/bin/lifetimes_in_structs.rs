#[derive(Debug)]
struct TrainSystem<'a> {
    name: &'a str,
}

fn main() {
    let name = String::from("AmTrak");
    let nj_transit = { TrainSystem { name: &name } };

    println!("{:#?}", nj_transit.name);
}
