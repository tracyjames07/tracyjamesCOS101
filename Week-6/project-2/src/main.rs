use std::io;

fn main() {
    
    let incentive1 = 500_000;
    let incentive2 = 800_000;
    let incentive3 = 1_000_000;
    let incentive4 = 100_000;

    let mut researcher_number = String::new();
    let mut researcher_name = String::new();
    let mut number_of_papers_published = String::new();

    println!("Welcome to the Nigerian Researchers Guide (NRG) RESEARCHERS PUBLICATION INCENTIVE SYSTEM (RPIS.)");
    println!("This will be used to generate the given incentive in terms of your works.");
    println!("Please input in the required data, so we can start.");

    println!("Enter your name: {}", researcher_name);
    io::stdin().read_line(&mut researcher_name).expect("Your input is invalid. Please try again.");

    println!("Enter your researcher number: {}", researcher_number);
    io::stdin().read_line(&mut researcher_number).expect("Your input is invalid. Please try again.");
    let mut researcher_number:i64 = researcher_number.trim().parse().expect("You input must me numerical. Please try again.");

    println!("How many papers have you published? {}", number_of_papers_published);
    io::stdin().read_line(&mut number_of_papers_published).expect("Your input is invalid. Please try again.");
    let mut number_of_papers_published:i64 = number_of_papers_published.trim().parse().expect("Your input must be numerical. Please try again.");

    if researcher_number > 500 {
        println!("Unfortunately, you cannot partake in this programme.")
    }

    while researcher_number <= 500 {

        if number_of_papers_published > 3 && number_of_papers_published < 5 {
            println!("Your incentive is N{}.", incentive1);
        }

        else if number_of_papers_published > 5 && number_of_papers_published < 10 {
            println!("Your incentive is N{}.", incentive2);
        }

        else if number_of_papers_published >= 10 {
            println!("Your incentive is N{}.", incentive3);
        }

        else if number_of_papers_published < 3 {
            println!("Your incentive is N{}.", incentive4);
        }

        break;
    }

}
