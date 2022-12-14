fn main() {
    let pattern = std::env::args().nth(1).expect("No Pattern Given");
    let path = std::env::args(2).expect("No Path Given");
    println!("Hello, world!");
}
