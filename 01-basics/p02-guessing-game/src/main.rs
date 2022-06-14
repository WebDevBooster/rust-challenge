extern crate rand;
extern crate colored;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;

fn main() {
    println!("Guess a number between 1 and 5!");
    println!("Enter your guess:");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=5);

        let mut guess_input = String::new();

        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line");

        let guess: u32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "YAY! You guessed correctly!".green().bold());
                break;
            },
        }

        println!("You guessed: {} The secret number was: {}", guess_input, secret_number);
    }

}
