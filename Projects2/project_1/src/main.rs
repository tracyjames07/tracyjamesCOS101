use std::io;

fn main() {
    println!("Calculating then speed of cars in kilometres.");

    let mut distance_in_miles = String::new();
    let mut time = String::new();

    println!("Please enter your distance in miles.");
        io::stdin().read_line(&mut distance_in_miles).expect("Your value is not a float.");
    let distance_in_miles:f32 = distance_in_miles.trim().parse().expect("Please try again.");
    println!("Distance in miles: {}", distance_in_miles);

    println!("Please enter your time in hours.");
        io::stdin().read_line(&mut time).expect("Your time is not a float.");
    let time:f32 = time.trim().parse().expect("Please try again.");
    println!("Time in hours: {}", time);

    let mut distance_in_kilometres = distance_in_miles *  1.609375;
    println!("Distance in kilometres is equal to {}", distance_in_kilometres);

    let mut speed_in_kilometres = distance_in_kilometres * time;
    println!("Speed in kilometres is equal to {}", speed_in_kilometres);
}