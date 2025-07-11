use std::io;
fn main() {
    println!("Welcome to number guessing game");
    println!("Guess a number");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");

    println!("You guessed {}",input);
}
