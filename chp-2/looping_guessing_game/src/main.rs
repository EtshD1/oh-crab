use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("-- Number Guessing Game (Looped) --");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("Please enter a number");

        match io::stdin().read_line(&mut guess) {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to read line. Please try again");
                continue;
            }
        };

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("Your guess is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
            Ordering::Less => println!("Try higher\n"),
            Ordering::Greater => println!("Try lower\n"),
        }
    }
}
