fn level_checker() {

    let aps1 = vec!["Intern", "Null", "Paralegal", "Placement"];
    let aps2 = vec!["Administrator", "Research Assistant", "Juniour Associate", "Classroom Teacher"];
    let aps3 = vec!["Senior Administrator", "PhD Assistant", "Associate", "Senior Teacher"];
    let el1 = vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"];
    let el2 = vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"];
    let ses = vec!["CEO", "Dean", "Partner", "Principal"];

}

fn intro() {

    println!("Welcome to the 'Public Service APS Level Checker' endorsed by the Federal Government of Nigeria.");
    println!("Please enter in the required information as the following.");
    println!("  
                1. Your fullname.
                2. Your date of birth.
                3. Your nationality, or country of origin.
                4. Your state of origin, if you are a Nigerian citizen.
                5. Your years of service. That is, how many years have you worked?
                6. Your current job.
            ");
    println!("Thank you.");

}

fn fullname() {

    let mut fullname = String::new();
    println!("Enter your full name: {}", fullname);
    std::io::stdin().read_line(&mut fullname).expect("Invalid input. Try again, please.");

}

fn date_of_birth() {

    println!("Enter your date of birth as required.");

    let mut dob_day = String::new();
    println!("Day: {}");
    std::io::stdin().read_line(&mut dob_day).expect("Invalid input. Try again, please.");
    let dob_day:i64 = dob_day.trim().parse().expect("Your input must be an integer.");

    let mut dob_month = String::new();
    println!("Month: {}");
    std::io::stdin().read_line(&mut dob_month).expect("Invalid input. Try again, please.");
    let dob_month:i64 = dob_month.trim().parse().expect("Your input must be an integer.");

    let mut dob_year = String::new();
    println!("Year: {}");
    std::io::stdin().read_line(&mut dob_year).expect("Invalid input. Try again, please.");
    let dob_year:i64 = dob_year.trim().parse().expect("Your input must be an integer.");

    println!("Date of Birth: {}/{}/{}.", dob_day, dob_month, dob_year);

}

fn nationality() {

    let mut nationality = String::new();
    println!("Enter your nationality: {}", nationality);
    std::io::stdin().read_line(&mut nationality).expect("Invalid input. Try again, please.");

}

fn state_of_origin() {

    let mut state_of_origin = String::new();
    println!("As you are Nigerian, enter your state of origin: {}", state_of_origin);
    std::io::stdin().read_line(&mut state_of_origin).expect("Invalid input. Try again, please.");

}

fn years_of_service() {

    let mut years_of_service = String::new();
    println!("How many years of work experience do you have?");
    println!("{}", years_of_service);
    std::io::stdin().read_line(&mut years_of_service).expect("Invalid input. Try again, please.");
    let years_of_service:i64 = years_of_service.trim().parse().expect("Your input must be an integer.");

}

fn current_job() {

    let mut current_job = String::new();
    println!("What is your current job?");
    println!("{}", current_job);
    std::io::stdin().read_line(&mut current_job).expect("Invalid input. Try again, please.");

}

fn main() {

    if current_job = aps1 && 