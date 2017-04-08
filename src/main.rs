extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut x = String::new();
    let mut y = String::new();

    println!("Guess the number!");

    println!("Choose range of number: ");
    println!("From: ");
    io::stdin().read_line(&mut x)
        .expect("Failed to read line");
    let x: u32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input a number!"), // TODO: Repeat input with error message.
    };
    println!("To: ");
    io::stdin().read_line(&mut y)
        .expect("Failed to read line");
    let y: u32 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input a number!"), // TODO: Repeat input with error message.
    };

    // Generate random number betwen 1 and 100.
    let secret_number = rand::thread_rng().gen_range(x, y);
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