fn main() {
    // what is ownership
    let x = 5; // x is valid for the rest of the scope
    {
        // x is valid here
        println!("{x}");
        let s = "hello";
        // is valid here
        println!("{} world", s);
    }
    // s is no longer valid

    let mut str = String::from("Anotha");

    str.push_str(" One");

    let str2 = str;
    // println!("{}", str); This doesn't work because str is no valid
    println!("{}", str2);

    // This is how to deep copy a variable
    let mut str3 = str2.clone();
    str3.push_str(". And anotha");
    println!("{}", str3);
    // Ownership only works on variables that are on the heap
    // On stack variables, the variable is copied directly
    let y = x;
    println!("y: {}, x: {}", y, x);

    // Owner and functions
    // Function parameters works the same as variable assignment
    // Passing in a variable in the is moving the variable
    // Passing a variable in the stack is coping the stack
    let wasnt_mine_now_it_is = gives_ownership();
    println!("{}", wasnt_mine_now_it_is);

    let this_was_mine = String::from("Wassaap");
    let now_it_is_mine = takes_and_give_back(this_was_mine);
    // println!("{}", this_was_mine); This doesn't work since variable 'this_was_mine' was moved
    println!("{}", now_it_is_mine);

    // If we need to return multiple values
    // it can be done as a tuple
    let (not_anymore, len) = calc_length(now_it_is_mine);
    println!("String '{}' is {len}", { not_anymore })
}

fn gives_ownership() -> String {
    let supposedly_mine = String::from("This string came from a function");
    supposedly_mine
}

fn takes_and_give_back(x: String) -> String {
    x
}

fn calc_length(str: String) -> (String, usize) {
    let len = str.len();
    (str, len)
}
