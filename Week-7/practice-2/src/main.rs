use std::io;

fn checker() {
    
    let mut character = String::new();

    println!("Enter a character: {}", character);
    io::stdin().read_line(&mut character).expect("Failed to read input.");
    let character:char = character.trim().parse().expect("Invalid input.");

    if character >= '0' && character <= '9' {
        println!("Character '{}' is a digit.", character);
    }

    else {
        println!("Character '{}' is not a digit.", character);
    }

}

fn main() {
    println!("Welcome! This program checks whether a character variable contains a digit or not.");
    checker()
}