#[allow(non_snake_case)]

fn _isDivisibleby(x: i64) -> i8 {
    if x % 2 == 0 {
        println!("{} is divisible by 2", x);
    } else if x % 3 == 0 {
        println!("{} is divisible by 2", x);
    } else if x % 5 == 0 {
        println!("{} is divisible by 2", x);
    } else if x % 7 == 0 {
        println!("{} is divisible by 2", x);
    } else {
        println!("{} could be a prime number", x);
    }
    return 0;
}

fn _runner() {
    print!("Welcome to the class of Conditionals in Rust");
}
