use std::io;

fn main() {

    println!("Enter a number: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let mut number:i32 = input1.trim().parse().expect("Failed to input value.");

    while number < 10 {

        println!("Inside the loop, number value is {}", number);
        number += 1;
    }
    println!("Outside the loop, number value is {}", number);
}