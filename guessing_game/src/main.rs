use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {

    println!("Guess the number");

    let secret_number = rand::rng().random_range(1..=100);

    println!("secret number is : {secret_number}");

    loop{
        println!("plz input your guess");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // let guess:u32 = guess.trim().parse().expect("not a number");
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("you guessed : {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
