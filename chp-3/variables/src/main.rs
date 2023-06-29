use std::io;

fn main() {
    let x = 5;
    println!("Value of x is {x}");
    let x = 6;
    println!("Value of x is {x}");

    let mut x: u8 = 0;

    loop {
        println!("X is {x}");
        let mut guess = String::new();

        print!("A or B: ");

        match io::stdin().read_line(&mut guess) {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to read line. Please try again");
                continue;
            }
        };

        match guess.trim().to_lowercase().as_str() {
            "a" => {
                x += 10;
                println!("X is now {x}");
            }
            _ => {
                break;
            }
        }
    }
}
