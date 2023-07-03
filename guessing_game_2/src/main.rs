use rand::Rng;

use std::cmp::Ordering;

fn main() {
    // Ask the user for input
    println!("Please enter a number between 1-20: ");
    // Create a new empty string instance
    let mut input = String::new();
    // Read the input from stdin
    std::io::stdin().read_line(&mut input).expect("Couldn't get input");
    // Trim and parse the input to an int
    let guess: i32 = input.trim().parse().expect("Couldn't convert to int");
    // Make random number with range 1-20
    let mut rng = rand::thread_rng();
    let answer: i32 = rng.gen_range(1..20);
    // Use match to compare the value
    match guess.cmp(&answer) {
        Ordering::Greater => println!("Too high"),
        Ordering::Less => println!("Too low"),
        Ordering::Equal => println!("Correct guess")
    }
}
