use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    //user game name prompt
    println!("----------Welcome to guessing game--------------");

    let secrate_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("enter your guesses 1 to 100: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Read unsuccessful");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You have to give the integer number only");
                continue;
            }
        };

        match guess.cmp(&secrate_number) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large!"),
        }
    }
}
