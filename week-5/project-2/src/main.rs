use std::io;

fn main() {
    
    let mut age = String::new();
    let mut experience = String::new();

    println!("Welcome, employee. How old are you? {}", age);
    io::stdin().read_line(&mut age).expect("Your input is invalid.");
    let mut age:i32 = age.trim().parse().expect("Your input has to be numerical. Please try again.");

    println!("Are you experienced in this career field? \nYes or No? {}", experience);
    io::stdin().read_line(&mut experience).expect("Your input is invalid. Please try again.");

    let incentive1:i32 = 18_720_000;
    let incentive2:i32 = 17_760_000;
    let incentive3:i32 = 15_600_000;
    let incentive4:i32 = 1_200_000;

    if experience == "No"
    {
        println!("As you are inexperienced in this field, your starting incentive is N{}.", incentive4);
    }
    else if experience == "Yes"
    {
        println!("Please proceed.");
    }

    if age >= 40
    {
        println!("Your annual incentive is N{}.", incentive1);
    }
    else if age >= 30 && age < 40
    {
        println!("Your annual incentive is N{}.", incentive2);
    }
    else if age <= 29 && age >= 16
    {
        println!("Your annual incentive is N{}.", incentive3);
    }
}