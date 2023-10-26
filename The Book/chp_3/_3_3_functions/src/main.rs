fn main() {
    // Function
    println!("Hello, Functions!");

    anotha_one(5);
    and_anotha(5, 'h');

    // Expressions and Statements
    let _x = 6; // This is a statement

    // This statement has a scope as an expression which is valid
    let _x = {
        let y = 5;
        y + 2
    };

    let x = five();

    println!("Value returned from function five is {x}");
}

fn anotha_one(x: u8) {
    println!("Anotha one");
    println!("Here is the parameter: {}", x);
}

fn and_anotha(x: u8, y: char) {
    println!("Remaining time is {x}{y}");
}

// function with a return statement
fn five() -> u8 {
    // Functions that end with a expression will return that expression
    // However, anding a semicolon will turn the expression into a statement
    // and thus the funtion will not return that statement
    5
}
