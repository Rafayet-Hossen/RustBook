use std::{cmp::Ordering, io};
use rand::Rng;
fn main() {
    println!("-----Gessing Game-----");
    
    let secrate_num = rand::thread_rng().gen_range(1..=100);
    
    println!("Enter your guesses.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Data is not readed successfully");

    let guess: u32 = guess.trim().parse().expect("Enter Numbers Only");

    println!("Your guessed: {}",guess);

    match guess.cmp(&secrate_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too large!"),
        Ordering::Equal => println!("You won!"),
    }

    println!("Machine guess {}",secrate_num);
}
