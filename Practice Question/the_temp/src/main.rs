use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("-----The Secrate Temperature-----");
    let secrate_temp = rand::thread_rng().gen_range(1..=50);

    let mut attempts = 0;

    loop {
        println!("Attempts {}/5", attempts+1);
        let mut guess_temp = String::new();

        println!("Give your guess temperature 1 to 50: ");

        io::stdin()
            .read_line(&mut guess_temp)
            .expect("Temperature read is not successful");

        let guess_temp: i32 = match guess_temp.trim().parse() {
            Ok(guess_num) => guess_num,
            Err(_) => {
                println!("Enter Number Between 1 to 50");
                continue;
            }
        };

        match guess_temp.cmp(&secrate_temp) {
            Ordering::Equal => {
                println!("Target temperature reached! You win! 🎉");
                break;
            }
            Ordering::Greater => {
                println!("Too hot! 🔥");
                attempts += 1;
            }
            Ordering::Less => {
                println!("Too cold! 🧊");
                attempts += 1;
            }
        }
        if attempts == 5 {
            println!("Game Over! You've run out of attempts. The correct temperature was: {secrate_temp}");
            break;
        }
    }
}
