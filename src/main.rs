use std::io;
use rand;
use std::cmp::Ordering;

fn main() {
    loop {
        println!("Guess the number!");
        println!("Please input your guess.");

        let mut guess = String::new();

        let random = rand::random_range(1..=100);

        io::stdin().read_line(&mut guess)
            .expect("Unable to read your input");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&random) {
            Ordering::Greater => println!("Your guess was bigger"),
            Ordering::Equal => { 
                println!("You made the correct Guess");
                break;
            },
            Ordering::Less => println!("Your guess was smaller")
        }

    }
    
}