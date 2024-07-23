use rand::prelude::*;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut guess = String::new();
        // io::stdin().read_line(&mut guess).expect("Failed to read input line.");
        match io::stdin().read_line(&mut guess) {
            Ok(s) => s,
            Err(_e) => {
                println!("Try again.");
                continue;
            }
        };
        let guess: u32 = match guess.trim().parse() {
            Ok(s) => s,
            Err(_e) => {
                println!("That's not a valid number. Try again.");
                continue;
            }
        };
        // let guess: u32 = guess.trim().parse().expect("Failed to parse the guess.");

        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
    }
}
