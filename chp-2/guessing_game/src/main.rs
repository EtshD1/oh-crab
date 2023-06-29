use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("-- Guess the number! --");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    play(secret_number);
}

fn play(number: u32) {
    println!("Please input your guess.");

    let mut guess = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to read line. Please try again");
            play(number);
        }
    };

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            play(number);
            return;
        }
    };

    println!("Your guess is: {guess}");

    match guess.cmp(&number) {
        Ordering::Less => {
            println!("Try Higher");
            play(number);
        }
        Ordering::Greater => {
            println!("Try Lower");
            play(number);
        }
        Ordering::Equal => println!("You got it!\nNumber was {number}"),
    }
}
