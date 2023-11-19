use std::io;

fn main() {

    let hedi1 = "Alzheimers";       //hedi = Health Diagnosis.
    let hedi2 = "Arrhythmia";
    let hedi3 = "Chronic Kidney Disease";
    let hedi4 = "Diabetes";
    let hedi5 = "Arthritis";

    let ahedi1:f64 = 1_200_000.00;      //ahedi = Amount for Health Diagnosis.
    let ahedi2:f64 = 550_000.00;
    let ahedi3:f64 = 1_500_000.00;
    let ahedi4:f64 = 800_000.00;
    let ahedi5:f64 = 450_000.00;

    let vil1 = "Akpabom";       //vil = Village of Residence.
    let vil2 = "Ngbauji";
    let vil3 = "Atabrikang";
    let vil4 = "Okorobilom";
    let vil5 = "Emeremen";

    println!("Welcome to OTUNENE FAMILY HEALTH CENTRE. Your health is our concern.");
    println!("We recently began remodelling the OPIS (OTUNENE PATIENT INFORMATION SYSTEM.)");
    println!("And for that reason, we will need your information");

    let mut entry_number = String::new();
    let mut name = String::new();
    let mut dob_day = String::new();
    let mut dob_month = String::new();
    let mut dob_year = String::new();
    let mut email_address = String::new();
    let mut phone_number = String::new();
    let mut number_of_siblings = String::new();
    let mut number_of_children = String::new();
    let mut medical_diagnosis = String::new();
    let mut village_of_residence = String::new();

    println!("Entry Number: {}", entry_number);
    io::stdin().read_line(&mut entry_number).expect("Your input is invalid. Please try again.");
    let mut entry_number:f64 = entry_number.trim().parse().expect("Your input must be numerical. Please try again.");

    println!("Name: {}", name);
    io::stdin().read_line(&mut name).expect("Your input is invalid. Please try again.");

    println!("Date of Birth: {}.{}.{}", dob_day, dob_month, dob_year);
    io::stdin().read_line(&mut dob_day).expect("Your input is invalid. Please try again.");
    io::stdin().read_line(&mut dob_month).expect("Your input is invalid. Please try again.");
    io::stdin().read_line(&mut dob_year).expect("Your input is invalid. Please try again.");
    let mut dob_day:f64 = dob_day.trim().parse().expect("Your input must be numerical. Please try again.");
    let mut dob_month:f64 = dob_month.trim().parse().expect("Your input must be numerical. Please try again.");
    let mut dob_year:f64 = dob_year.trim().parse().expect("Your input must be numerical. Please try again.");

    println!("Email Address: {}", email_address);
    io::stdin().read_line(&mut email_address).expect("Your input is invalid. Please try again.");

    println!("Phone Number: {}", phone_number);
    io::stdin().read_line(&mut phone_number).expect("Your input is invalid. Please try again.");
    let mut phone_number:f64 = phone_number.trim().parse().expect("Your input must be numerical. Please try again.");

    println!("Number of Siblings: {}", number_of_siblings);
    io::stdin().read_line(&mut number_of_siblings).expect("Your input is invalid. Please try again.");
    let mut number_of_siblings:f64 = number_of_siblings.trim().parse().expect("Your input must be numerical. Please try again.");

    println!("Number of Children: {}", number_of_children);
    io::stdin().read_line(&mut number_of_children).expect("Your input is invalid. Please try again.");
    let mut number_of_children:f64 = number_of_children.trim().parse().expect("Your input must be numerical. Please try again.");

    println!("Medical Diagnosis: {}", medical_diagnosis);
    io::stdin().read_line(&mut medical_diagnosis).expect("Your input is invalid. Please try again.");

    println!("Village of Residence: {}", village_of_residence);
    io::stdin().read_line(&mut village_of_residence).expect("Your input is invalid. Please try again.");

    let year:f64 = 2023.00;
    let mut age:f64 = year - dob_year;

    println!("You are {}.", age);

    while entry_number <= 100.00 {

        if medical_diagnosis == hedi1 && age > 50.00 && number_of_children > 4.00 && village_of_residence == vil1 {
            println!("Congratulations! You are eligible for a 20% discount!");
            let d1:f64 = 0.20;      //d = Discount.
            let dcost1:f64 = d1 * ahedi1;       //dcost = Discounted Cost.
            println!("Your calculated discount is N{}.", dcost1);
        }

        else if medical_diagnosis == hedi2 && age == 30.00 && number_of_siblings > 4.00 && village_of_residence == vil2 {
            println!("Congratulations! You are eligible for a 5% discount!");
            let d2:f64 = 0.05;
            let dcost2:f64 = d2 * ahedi2;
            println!("Your calculated discount is N{}.", dcost2);
        }

        else if medical_diagnosis == hedi3 && age > 40.00 && number_of_children > 3.00 && number_of_siblings > 3.00 && village_of_residence == vil3 {
            println!("Congratulations! You are eligible for a 15% discount!");
            let d3:f64 = 0.15;
            let dcost3:f64 = d3 * ahedi3;
            println!("Your calculated discount is N{}.", dcost3);
        }

        else if medical_diagnosis == hedi4 && age > 28.00 && age < 45.00 && number_of_children > 2.00 && number_of_children <= 4.00 && village_of_residence == vil4 {
            println!("Congratulations! You are eligible for a 10% discount!");
            let d4:f64 = 0.10;
            let dcost4:f64 = d4 * ahedi4;
            println!("Your calculated discount is N{}.", dcost4);
        }

        else if medical_diagnosis == hedi5 && age > 58.00 && number_of_siblings > 5.00 && number_of_children > 5.00 && village_of_residence == vil5 {
            println!("Congratulations! You are eligible for a 10% discount!");
            let d5:f64 = 0.10;
            let dcost5:f64 = d5 * ahedi5;
            println!("Your calculated discount is N{}.", dcost5);
        }
    }

    while entry_number > 100.00 {
        println!("You are the {}th patient. Therefore, you are not eligible for a discount.", entry_number);

        if medical_diagnosis == hedi1 {
            println!("Your treatment costs N{}.", ahedi1);
        }

        else if medical_diagnosis == hedi2 {
            println!("Your treatment costs N{}.", ahedi2);
        }

        else if medical_diagnosis == hedi3 {
            println!("Your treatment costs N{}.", ahedi3);
        }

        else if medical_diagnosis == hedi4 {
            println!("Your treatment costs N{}.", ahedi4);
        }

        else if medical_diagnosis == hedi5 {
            println!("Your treatment costs N{}.", ahedi5);
        }
    }
}
