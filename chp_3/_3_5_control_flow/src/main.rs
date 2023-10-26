use rand::Rng;

fn main() {
    // Control flows is the ability to run code with specific conditions
    println!("Hello, world!");

    // if Expressions
    let mut x: i32 = rand::thread_rng().gen_range(0..10);
    if x == 5 {
        // condition to check if x is equal 5
        println!("x is equal to 5");
    } else if x > 5 {
        // if the first conditions fails, check if x is greater than 5
        println!("x is greater than 5");
    } else {
        // if all conditions fail, print this statement
        println!("x  is less than 5");
    }

    // using if as expression
    let mut y = if x == 5 { 50 } else { 10 };
    println!("y is now {y}");

    // Loops
    'main_loop: loop {
        println!("x is {x}");
        loop {
            if x <= 0 {
                break 'main_loop;
            }
            println!("Anotha one!!");
            if y <= 0 {
                x -= 1;
                break;
            }
            y -= 1;
        }
    }

    let arr = [5, 3, 7, 1, 0];
    for i in 0..arr.len() {
        println!("array element {} is {}", { i }, { arr[i] });
    }

    let mut z = 10;
    while z > 0 {
        println!("Z is now {z}");
        z -= 1;
    }

    for i in arr {
        println!("arr has element {i}")
    }

    for i in (1..=10).rev() {
        println!("{i}!");
    }
    println!("LIFTOFF!!!")
}
