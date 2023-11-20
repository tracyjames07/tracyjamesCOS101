use std::io;

fn main() {

    println!("MULTIPLICATION TABLE GENERATOR.");
    println!("Input in required values to begin.");

    let mut number = String::new();
    println!("Enter your maximum factor: {}", number);
    io::stdin().read_line(&mut number).expect("Invalid input. Try again, please.");
    let mut number:i32 = number.trim().parse().expect("Your input has to be an integer.");

    for factor in 1..=number {
        for integer in 1..=20 {
            let multiple = factor * integer;
            println!("{} x {} = {}", factor, integer, multiple);
        }
        println!("");
    }
    
}