extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut rand_x = String::new();
    let mut rand_y = String::new();

    println!("Guess the number!");

    // Player input of random number generation.
    println!("Choose range of number: ");
    println!("From: ");
    io::stdin()
        .read_line(&mut rand_x)
        .expect("Failed to read line");
    let rand_x: u32 = match rand_x.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input a number!"), // TODO: Repeat input with error message.
    };
    println!("To: ");
    io::stdin()
        .read_line(&mut rand_y)
        .expect("Failed to read line");
    let rand_y: u32 = match rand_y.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input a number!"), // TODO: Repeat input with error message.
    };

    // Generate random number from player input.
    let random_number = rand::thread_rng().gen_range(rand_x, rand_y);
    // println!("The secret number is: {}", random_number);
    let mut tries = 0;

    loop {
        println!("Tries: {}", &tries);
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => {
                println!("Too small!");
                tries += 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                tries += 1;
            }

            Ordering::Equal => {
                tries += 1;
                println!("You win!");
                println!("You tried: {} times.", &tries);
                break;
            }
        }

    }

}
