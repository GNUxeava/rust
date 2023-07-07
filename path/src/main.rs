use std::io::{stdin, Write};
use std::fs::File;

fn main() {
    let mut path = String::new();
    println!("Enter file path: ");
    stdin().read_line(&mut path).unwrap();
    let mut file = File::create(&path.trim()).unwrap();

    let mut content = String::new();
    println!("Enter content for file: ");
    stdin().read_line(&mut content).unwrap();
    file.write_all(&content.trim().as_bytes()).unwrap();
}
