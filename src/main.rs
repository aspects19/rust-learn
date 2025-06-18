use std::io;
use rand;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    let random = rand::random_range(1..=100);

    io::stdin().read_line(&mut guess)
        .expect("Unable to read your input");

    if random.to_string() == guess {
        println!("You won!\n The correct number was {}", guess)
    } else {
        println!("Your guess was {} and computer guessed {}", guess, random)
    }

}