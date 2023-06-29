fn main() {
    let mut number = 3;

    'temp_while: while number != 0 {
        println!("{number}!");

        number -= 1;

        if number >= 10 {
            break 'temp_while;
        }
    }

    println!("LIFTOFF!!!");
}
