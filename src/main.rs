extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate random number betwen 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    let mut tries = 0;

    loop {
        println!("Tries: {}", tries);
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                tries = tries +1;
            }
            Ordering::Greater => {
                println!("Too big!");
                tries = tries +1;
            }

            Ordering::Equal => {
                tries = tries +1;
                println!("You win!");
                println!("You tried: {} times.", tries);
                break;
            }
        }

    }

}
