use std::env;

fn main() {
    let name = env::args().nth(1).unwrap_or("Nobody".to_string());
    println!("Hello {}", name)
}