extern crate rand;

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret Number is: {}", secret_number);

    loop {
        println!("Please input your guess. ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Fail to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You Guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You WIN!");
                break;
            }
        }
    }
}