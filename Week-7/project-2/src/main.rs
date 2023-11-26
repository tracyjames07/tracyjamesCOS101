use std::io;

fn sibling_name() {

    let mut sibling_name = String::new();
    println!("Enter your sibling's first name: {}", sibling_name);
    io::stdin().read_line(&mut sibling_name).expect("Invalid input. Try again, please.");

}

fn sibling_age() {

    let mut sibling_age = String::new();
    println!("Enter your sibling's age: {}", sibling_age);
    io::stdin().read_line(&mut sibling_age).expect("Invalid input. Try again, please.");
    let sibling_age:i64 = sibling_age.trim().parse().expect("Their age must be a positive integer.");

}

fn marital_status() {

    let mut marital_status = String::new();
    println!("What is their marital status? {}", marital_status);
    io::stdin().read_line(&mut marital_status).expect("Invalid input. Try again, please.");

}

fn career_status() {

    let mut career_status = String::new();
    println!("As your sibling is unmarried, please tell us their career status.");
    println!("{}", career_status);
    io::stdin().read_line(&mut career_status).expect("Invalid input. Try again, please.");

}

fn student_information() {

    println!("As your sibling is unmarried, we will need their student information.");
    println!("Do well to ensure your inputed data is correct.");

    let mut university = String::new();
    println!("What university do they attend? {}", university);
    io::stdin().read_line(&mut university).expect("Invalid input. Try again, please.");

    let mut course_of_study = String::new();
    println!("What field are they pursuing a degree in? {}", course_of_study);
    io::stdin().read_line(&mut course_of_study).expect("Invalid input. Try again, please.");

}

fn marital_information() {

    println!("As your sibling is married, we will need their marital information.");
    println!("Do well to ensure your inputed data is correct.");

    let mut offspring = String::new();
    println!("Do they have any offspring? {}", offspring);
    io::stdin().read_line(&mut offspring).expect("Invalid input. Try again, please.");

    let mut city_of_residence = String::new();
    println!("What city does your sibling and their family reside in? {}", city_of_residence);
    io::stdin().read_line(&mut city_of_residence).expect("Invalid input. Try again, please.");

}

fn education_status() {

    let mut waec_status = String::new();
    println!("Has your sibling written the WAEC exams? {}", waec_status);
    io::stdin().read_line(&mut waec_status).expect("Invalid input. Try again, please.");

    let mut secondary_school = String::new();
    println!("What secondary school did they attend? {}", secondary_school);
    io::stdin().read_line(&mut secondary_school).expect("Invalid input. Try again, please.");

    let mut class_level = String::new();
    println!("What class are they currently in? {}", class_level);
    io::stdin().read_line(&mut class_level).expect("Invalid input. Try again, please.");
    
}