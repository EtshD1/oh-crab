use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing Game!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        match io::stdin().read_line(&mut guess) {
            Ok(_) => {},
            Err(_)=> {
                println!("Failed to read line");
                continue
            },
        };

        println!("Your guess is: {}", guess);

        let guess = match guess.trim().parse::<i32>() {
            Ok(expr) => expr,
            Err(_)=> {
                println!("Please use a valid number");
                continue
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => {
                println!("Try lower");
            },
            Ordering::Less => {
                println!("Try higher");
            },
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
        };
    }

    println!("You won the game!");
}
