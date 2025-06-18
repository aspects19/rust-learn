use std::io;
use rand;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    let random = rand::random_range(1..=100);

    io::stdin().read_line(&mut guess)
        .expect("Unable to read your input");

    println!("You guessed {}", guess);
    println!("Computer guessed {}", random);

}