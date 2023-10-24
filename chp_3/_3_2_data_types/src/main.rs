use std::num::Wrapping;
use std::io;

fn main() {
    // Data Types
    println!("Data type example");
    println!("Decimals:");
    let mut x: u8 = 0;

    x += 1;

    println!("x: {}", x);

    println!();
    // Numerical Systems
    println!("Numerical Systems examples");
    let dec = 98_222;
    println!("Decimal (98_222): {}", dec);

    let hex = 0xff;
    println!("Hexadecimal (0xff): {}", hex);

    let oct = 0o77;
    println!("Octal (0o77): {}", oct);

    let bin = 0b1111_0001;
    println!("Binary (0b1111_0001): {}", bin);

    println!();
    // Overflowing/Wrapping
    overflowing_x();

    println!();
    // Tuple
    println!("Tuple example");
    println!("Creating a (String, u8, bool) tuple");
    let tup: (String, u8, bool) = ("Hello, World".to_string(), 7, false);

    println!("Tuple Element 0: {}", tup.0);
    println!("Tuple Element 1: {}", tup.1);
    println!("Tuple Element 2: {}", tup.2);

    println!("Destructering Tuble into a, b, c");
    let (a, b, c) = tup;

    println!("Value a: {}", a);
    println!("Value b: {}", b);
    println!("Value c: {}", c);

    println!();
    // Arrays
    println!("Array Example");
    let arr:[u8; 10] = [0, 1, 2, 3, 4, 6, 7, 8, 9, 10];
    println!("First index of the array is {}", arr[0]);

    for i in 0..10 {
        println!("Index of {} is {}", i, arr[i]);
    }

    println!("Anotha one");
    let arr = [3;5];

    for i in 0..5 {
        println!("Index of {} is {}", i, arr[i]);
    }
    println!();

    // THIS CODE PANICS IF YOU ENTER A NUMBER PAST 4
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn overflowing_x () -> () {
    let mut x  = Wrapping(0u8);
    println!("Integer overflowing example");
    println!("Starting with an initialized value of x = {x}");

    println!("Subtracting 1");
    x -= 1;
    println!("x is now {x}");
}
