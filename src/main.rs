use std::io;

fn main() {
    println!("Press any key to say hello...");
    io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    println!("Hello, world!");
}