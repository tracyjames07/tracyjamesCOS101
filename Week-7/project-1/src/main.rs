use std::io;

fn main() {

    println!("Welcome to the AREA & VOLUME CALCULATOR.");
    println!("Select a number from one to five to start.");
    println!("

                           AVAILABLE FUNCTIONS.

                        A. Area of a Trapezium. 
                        B. Area of a Rhombus. 
                        C. Area of a parallelogram. 
                        D. Area of a Cube. 
                        E. Volume of a Cylinder.

        ");

    println!("Enter a letter to select a function and start the sequence.");

    let mut input = String::new();
    println!("{}", input);
    io::stdin().read_line(&mut input).expect("Invalid input. Try again, please.");
    let input =input.trim().to_uppercase();

    if input == "A" {
        area_of_trapezium();
    }

    else if input == "B" {
        area_of_rhombus();
    }

    else if input == "C" {
        area_of_parallelogram();
    }

    else if input == "D" {
        area_of_cube();
    }

    else if input == "E" {
        volume_of_cylinder();
    }

    else {
        println!("Invalid input.");
    }
}

fn area_of_trapezium() {
    
    let mut height = String::new();
    println!("Enter a value for height: {}", height);
    io::stdin().read_line(&mut height).expect("Invalid input. Try again, please.");
    let height:f64 = height.trim().parse().expect("Your input must be a float.");

    let mut base1 = String::new();
    println!("Enter a value for base one: {}", base1);
    io::stdin().read_line(&mut base1).expect("Invalid input. Try again, please.");
    let base1:f64 = base1.trim().parse().expect("Your input must be a float.");

    let mut base2 = String::new();
    println!("Enter a value for base two: {}", base2);
    io::stdin().read_line(&mut base2).expect("Invalid input. Try again, please.");
    let base2:f64 = base2.trim().parse().expect("Your input must be a float.");

    let area_of_trapezium:f64 = (height / 2.00) * (base1 + base2);
    println!("The area of the trapezium is {:.2}.", area_of_trapezium);

}

fn area_of_rhombus() {

    let mut diagonal1 = String::new();
    println!("Enter a value for diagonal one: {}", diagonal1);
    io::stdin().read_line(&mut diagonal1).expect("Invalid input. Try again, please.");
    let diagonal1:f64 = diagonal1.trim().parse().expect("Your input must be a float.");

    let mut diagonal2 = String::new();
    println!("Enter a value for diagonal two: {}", diagonal2);
    io::stdin().read_line(&mut diagonal2).expect("Invalid input. Try again, please.");
    let diagonal2:f64 = diagonal2.trim().parse().expect("Your input must be a float.");

    let area_of_rhombus:f64 = 0.50 * diagonal1 * diagonal1;
    println!("The area of the rhombus is {:.2}.", area_of_rhombus);

}

fn area_of_parallelogram() {

    let mut base = String::new();
    println!("Enter a value for the base: {}", base);
    io::stdin().read_line(&mut base).expect("Invalid input. Try again, please.");
    let base:f64 = base.trim().parse().expect("Your input must be a float.");

    let mut altitude = String::new();
    println!("Enter a value for the altitude: {}", altitude);
    io::stdin().read_line(&mut altitude).expect("Invalid input. Try again, please.");
    let altitude:f64 = altitude.trim().parse().expect("Your input must be a float.");

    let area_of_parallelogram:f64 = base * altitude;
    println!("The area of the parallelogram is {:.2}.", area_of_parallelogram);

}

fn area_of_cube() {

    let mut length_of_side = String::new();
    println!("Enter a value for length: {}", length_of_side);
    io::stdin().read_line(&mut length_of_side).expect("Invalid input. Try again, please.");
    let length_of_side:f64 = length_of_side.trim().parse().expect("Your input must be a float.");

    let area_of_cube = 6.00 * (length_of_side.powf(2.00));
    println!("The area of the cube is {:.2}.", area_of_cube);

}

fn volume_of_cylinder() {

    let mut radius = String::new();
    println!("Enter a value for radius: {}", radius);
    io::stdin().read_line(&mut radius).expect("Invalid input. Try again, please.");
    let radius:f64 = radius.trim().parse().expect("Your input must be a float.");

    let mut height = String::new();
    println!("Enter a value for height: {}", height);
    io::stdin().read_line(&mut height).expect("Invalid input. Try again, please.");
    let height:f64 = height.trim().parse().expect("Your input must be a float.");

    let pi:f64 = 3.141592654;


    let volume_of_cylinder:f64 = pi * (radius.powf(2.00)) * height;
    println!("The volume of the cylinder is {:.2}.", volume_of_cylinder);

}