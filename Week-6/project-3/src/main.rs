use std::io;

fn main() {

    println!("Input in required values to begin.");

    let mut minimum_value = String::new();
    println!("Enter your minimum value: {}", minimum_value);
    io::stdin().read_line(&mut minimum_value).expect("Invalid input. Try again, please.");
    let mut minimum_value:i32 = minimum_value.trim().parse().expect("Your input has to be an integer.");

    let mut maximum_value = String::new();  //This is what number your factor is multiplied by before the table ends.
    println!("Enter your maximum value: {}", maximum_value);
    io::stdin().read_line(&mut maximum_value).expect("Invalid input. Try again, please.");
    let mut maximum_value:i32 = maximum_value.trim().parse().expect("Your input has to be an integer.");

    let mut factor = String::new();
    println!("{}", factor);
    io::stdin().read_line(&mut factor).expect("Invalid input. Try again, please.");
    let mut factor:i32 = factor.trim().parse().expect("Your input has to be an integer.");

    while factor >= 1 {
        loop {
            for multiple in minimum_value..maximum_value {
                factor += factor;
                println!("Multiples are: {}", factor);

                if multiple > 25 {
                    break;
                }
            }
        }
    }
}