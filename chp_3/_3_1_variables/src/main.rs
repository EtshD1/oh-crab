// Constants
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {

    // Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("Now the value of x is: {x}");

    // Constants
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let y = 1;
    println!("Non-shadowed value: {}", y);
    {
        let y = y + 2;
        println!("Shadowed value in scope: {}", y);
    }
    let y = y + 1;
    println!("Shadowed value: {}", y);
}
