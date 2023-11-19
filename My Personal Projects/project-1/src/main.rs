use std::io;

fn main() {
    
    println!("Welcome to Mama's Kitchen!");
    println!("We ensure your taste buds are dancing in delight by the time you're done dining!");
    println!("And that's all thanks to Mama's special recipes!");

    println!("Would you like to see today's menu?");

    let mut response = String::new();

    println!("{}", response);
    io::stdin().read_line(&mut response).expect("I do not understand your response. Could you please come again?");

    if response == "No"{
        println!("Alright! Each one of us is ready to cater your needs if necessary. Have a great time.");
    }

    else if response == "Yes"{
        println!("Great! Here it is!");
    }
}
