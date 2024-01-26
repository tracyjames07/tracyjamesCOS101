use std::io;
use std::io::Read;

fn main() {

    println!("Welcome to the Globacom Database. 
              Please input what category from the following you fall under.

                    a. Administrator.
                    b. Project Manager.
                    c. Employee.
                    d. Customer.
                    e. Vendor.

            ");

    let mut input = String::new();
    println!("\nEnter your category: ");
    io::stdin().read_line(&mut input).expect("Not a valid string.");
    let input = input.trim();
    
    if input == "Administrator"
    {
        let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }
    else if input == "Project Manager"
    {
        let mut file = std::fs::File::open("project_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }
    else if input == "Employee"
    {
        let mut file = std::fs::File::open("staff_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }
    else if input == "Customer"
    {
        let mut file = std::fs::File::open("customer_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }
    else if input == "Vendor"
    {
        let mut file = std::fs::File::open("dataplan_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }

}