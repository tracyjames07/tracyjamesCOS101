fn main() {
    
    let fullname = "Tracy Owezie James";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";

    let mut school = "School of Science".to_string();
    school.push_str(" and Technology");

    println!("My name is: {}", fullname);
    println!("The length of my fullname is: {}", fullname.len());
    println!("I am a student of {} department", department);
    println!("{}", school);
    println!("{}", uni)
}