use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("{}", "Guess the number!".blue());

    let secret_number = rand::thread_rng().gen_range(1, 100);

    let mut count = 0;
    println!("{}", "Please, input your guess!".blue());
    loop {
        count += 1;
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("{}", "Please, type a number now okay?:".red());
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small dude, another one:".red()),
            Ordering::Greater => println!("{}", "Oh, you passed dude, try another one now:".red()),
            Ordering::Equal => {
                println!("{}",  format!("OMG, you win! your count of tries is: {}", count).green());
                break;
            }
        }
    }
}
