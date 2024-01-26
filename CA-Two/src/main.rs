use std::io;
use std::io::Write;
use std::fs::OpenOptions;

struct Company {

    name: String,
    year: i32,
    shares: f32,
    liabilities: f32,

}

impl Company {

    fn percentage_leverage(&self) -> f32 {

        ((self.shares - self.liabilities) / self.shares) * 100.00

    }

}

fn main() {

    let companies = vec![

    Company { name: String::from("Cadbury Nigeria Plc."), year: 1965, shares: 15_000_000.00, liabilities: 5_500_000.00},
    Company { name: String::from("Champion Breweries Plc."), year: 1974, shares: 25_000_000.00, liabilities: 8_000_000.00},
    Company { name: String::from("Dangote Sugar Refinery Plc."), year: 1970, shares: 18_000_000.00, liabilities: 10_000_000.00},
    Company { name: String::from("Flour Mills Nigeria Plc."), year: 1960, shares: 32_000_000.00, liabilities: 4_000_000.00},
    Company { name: String::from("Nestle Nigeria Plc."), year: 1961, shares: 8_000_000.00, liabilities: 1_500_000.00},
    Company { name: String::from("Unilever Nigeria Plc."), year: 1923, shares: 37_000_000.00, liabilities: 11_000_000.00},
    Company { name: String::from("Honeywell Nigeria Plc."), year: 1906, shares: 34_000_000.00, liabilities: 9_000_000.00},
    Company { name: String::from("Nigerian Breweries Plc."), year: 1946, shares: 30_000_000.00, liabilities: 12_000_000.00},

                        ];
        
    let mut company_name = vec!["\nCOMPANY", "\nCadbury", "\nChampion", "\nDangote Sugar", "\nFlour Mills of Nigeria", "\nNestle", "\nUnilever", "\nHoneywell", "\nNigerian Breweries"];
    let mut founding_year = vec!["\t\t\t\tFOUNDING YEAR", "\t\t\t1965", "\t\t\t1974", "\t\t\t1970", "\t\t\t1960", "\t\t\t1961", "\t\t\t1923", "\t\t\t1906", "\t\t\t1946"];
    let mut company_shares = vec!["\t\t\tSHARES", "\t\t15,000,000", "\t\t25,000,000", "\t\t18,000,000", "\t\t32,000,000", "\t\t8,000,000", "\t\t37,000,000", "\t\t34,000,000", "\t\t30,000,000"];
    let mut company_liabilities = vec!["\t\t\tLIABILITIES", "\t\t\t5,500,000", "\t\t\t8,000,000", "\t\t\t10,000,000", "\t\t\t4,000,000", "\t\t\t1,500,000", "\t\t\t11,000,000", "\t\t\t9,000,000", "\t\t\t12,000,000"];

    let mut sgm = std::fs::File::create("Springate Model.txt").expect("Unable to create file.");

    sgm.write_all("Springate Model (Z Score)".as_bytes()).expect("Unable to write into the file.");

    for m in 0..company_name.len() {

        sgm.write_all(company_name[m].as_bytes()).expect("Unable to write into the file.");

        sgm.write_all(founding_year[m].as_bytes()).expect("Unable to write into the file.");

        sgm.write_all(company_shares[m].as_bytes()).expect("Unable to write into the file.");

        sgm.write_all(company_liabilities[m].as_bytes()).expect("Unable to write into the file.");
    }

    println!("Enter your username.");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Unable to read entry.");
    println!("Your username is '{}'.", username);

    if username.len() >= 4
    {
        println!("Your username is valid. You may continue.");
    }
    else
    {
        println!("Your username is invalid. Try again.");
    }

    println!("Enter your password.");
    println!("\n
              Note that your password must be a minimum of three characters and a maximum of eight.
              Also, it must not contain uppercase letters, e.g. 'A', or special characters such as '$', '#', and '@'.
              Thank you.
              \n
            ");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Unable to read entry.");
    println!("Your password is '{}'.", password);

    if password.len() > 3 && password.len() < 8
    {
        println!("Your password is valid. You may continue.");
    }
    else if password.len() > 3 && password.len() == 8
    {
        println!("Your password is valid. You may continue.");
    }
    else if password.len() < 3
    {
        println!("Your password is invalid. Try again.");
    }
    else if password.len() > 8
    {
        println!("Your password is invalid. Try again.");
    }

    println!("Please do not share this information with anyone, as it is crucial to accessing sensitive data.");

    for company in &companies {

        let leverage = company.percentage_leverage();

        if company.shares > 20_000_000.00 {

            let leverage_multiple = leverage * 2.00;
            writeln!(sgm, "The multiple of the percentage leverage is {:.2}.", leverage_multiple).expect("Unable to write into the file.");

        }

        if company.liabilities < 10_000_000.00 {

            let five_percent = 0.05 * leverage;
            writeln!(sgm, "5% of the percentage leverage is {:.2}.", five_percent).expect("Unable to write into the file.");

        }

    }

    println!("Your login was successful!");
    println!("Your data has been saved."); 

}
