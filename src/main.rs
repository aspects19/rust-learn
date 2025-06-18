use std::io;
use rand;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    let random = rand::random_range(1..=100);

    io::stdin().read_line(&mut guess)
        .expect("Unable to read your input");

    match guess.cmp(&random.to_string()) {
        Ordering::Greater => println!("Your guess was bigger"),
        Ordering::Equal => println!("You made the correct Guess"),
        Ordering::Less => println!("Your guess was smaller")
    }

}