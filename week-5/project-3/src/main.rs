use std::io;

fn main() {
    let p = "Pounded Yam and Edinkaiko Soup";
    let f = "Fried Rice and Chicken";
    let a = "Amala and Ewedu Soup";
    let e = "Eba and Egusi Soup";
    let w = "White Rice and Stew";

    let pp:f32 = 3_200.00;
    let pf:f32 = 3_000.00;
    let pa:f32 = 2_500.00;
    let pe:f32 = 2_000.00;
    let pw:f32 = 2_500.00;

    println!("Welcome to MAMA'S KITCHEN! Your one stop to delicious food!");
    println!("Here's our menu.");



    println!("                 
                                                    MENU

                                Amala and Ewedu Soup                N2_500
                                Fried Rice and Chicken              N3_000
                                White Rice and Stew                 N2_500
                                Eba and Egusi Soup                  N2_000
                                Pounded Yam and Edinkaiko Soup      N3_200

                                            ATTENTION CUSTOMERS!
                            WITH PURCHASES ABOVE N10_000, YOU GET A 5% DISCOUNT!
            ");

    let mut food = String::new();

    println!("What would you like to eat today?");
    io::stdin().read_line(&mut food).expect("Your response wasn't valid. Perhaps, try putting in the correct response next time?"); 
    let food = food.trim();

    if food == "Pounded Yam and Edinkaiko Soup"{
        println!("How many portions of {} would you like to order?", p);
    }
    
    else if food == "Fried Rice and Chicken"{
        println!("How many portions of {} would you like to order?", f);
    }

    else if food == "Amala and Ewedu Soup"{
        println!("How many portions of {} would you like to order?", a);
    }

    else if food == "Eba and Egusi Soup"{
        println!("How many portions of {} would you like to order?", e);
    }

    else if food == "White Rice and Stew"{
        println!("How many portions of {} would you like to order?", w);
    }

    let mut quantity = String::new();

    println!("Input your preferred number of portions here.");
    println!("{}", quantity);
    io::stdin().read_line(&mut quantity).expect("Your input must be numerical.");
    let mut quantity:f32 = quantity.trim().parse().expect("Your input is invalid. Please try again.");

    let mut cost:f32 = 0.00;

    if food == "Pounded Yam and Edinkaiko Soup"{
        let price = pp * quantity;
        println!("Your total cost is N{}.", price);
        cost += price;
    }

    else if food == "Fried Rice and Chicken"{
        let price = pf * quantity;
        println!("Your total cost is N{}.", price);
        cost += price;
    }

    else if food == "Amala and Ewedu Soup"{
        let price = pa * quantity;
        println!("Your total cost is N{}.", price);
        cost += price;
    }

    else if food == "Eba and Egusi Soup"{
        let price = pe * quantity;
        println!("Your total cost is N{}.", price);
        cost += price;
    }
    

    else if food == "White Rice and Stew"{
        let price = pw * quantity;
        println!("Your total cost is N{}.", price);
        cost += price;
    }

    if cost > 10000.00{
        println!("Your order costs more than N10,000; so you get a discount!");
        let discount:f32 = cost - (cost * 0.05);
        println!("Your discounted cost is N{}.", discount);
    }

    println!("Thank you for your purchase! We hope you enjoy your meal!");

}