fn main() {
    let tup: (i32, bool, String) = (64, false, "Data Type".to_string());

    let (x, y, z) = tup;

    println!("Integer: {}\nBool: {}\nString: {}", x, y, z);

    let arr: [i32; 3] = [1, 2, 3];

    let first = arr[0];
    let second = arr[1];
    let thrid = arr[2];

    let temp = arr[0];

    println!("Array Contains: {}, {}, and {}", first, second, thrid);

    println!("First Element is {temp}");
}
