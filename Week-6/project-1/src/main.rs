use std::io;

fn main() {
    
    let mut candidate_number = String::new();
    let mut class_representative = String::new();
    let mut current_level = String::new();
    let mut cgpa = String::new();
    let mut name = String::new();
    let mut email = String::new();
    let mut department = String::new();
    let mut state_of_origin = String::new();

    println!("Welcome to the Pan-Atlantic University STUDENT COUNCIL VOTING SYSTEM.");
    println!("Please input in the required information to get started.");

    println!("Enter your name: {}", name);
    io::stdin().read_line(&mut name).expect("Invalid input. Try again, please.");

    println!("Enter your current level: {}", current_level);
    io::stdin().read_line(&mut current_level).expect("Invalid input. Try again, please.");
    let mut current_level:f64 = current_level.trim().parse().expect("Your input should be numerical.");

    println!("Enter your candidate number: {}", candidate_number);
    io::stdin().read_line(&mut candidate_number).expect("Invalid input. Try again, please.");
    let mut candidate_number:f64 = candidate_number.trim().parse().expect("Your input should be numerical.");

    println!("Enter your department of studies: {}", department);
    io::stdin().read_line(&mut department).expect("Invalid input. Try again, please.");

    println!("Enter your CGPA: {}", cgpa);
    io::stdin().read_line(&mut cgpa).expect("Invalid input. Try again, please.");
    let mut cgpa:f64 = cgpa.trim().parse().expect("Your input should be numerical.");

    println!("Are you a class representative? {}", class_representative);
    io::stdin().read_line(&mut class_representative).expect("Invalid input. Try again, please.");

    println!("Enter your email: {}", email);
    io::stdin().read_line(&mut email).expect("Invalid input. Try again, please.");

    println!("Enter your state of origin: {}", state_of_origin);
    io::stdin().read_line(&mut state_of_origin).expect("Invalid input. Try again, please.");
    println!("This is required majorly for statistics.");

    while candidate_number <= 150.00 {

        if class_representative == "Yes" && current_level >= 200.00 && cgpa > 4.00 {
            println!("Your name is {}; your email address is {}; you are currently studying in the {} department, and you are from {} state.", name, email, department, state_of_origin);
            println!("You are eligible to vote.");
        }

        else {
            println!("Sorry. You are not eligible to vote.");
        }

        break;
    }
}
