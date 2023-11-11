use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("We're solving for the roots of a quadratic equation.");

    println!("Enter a value for a: ");
    io::stdin().read_line(&mut a).expect("Your input is not a valid string.");
    let a:f32 = a.trim().parse().expect("Your input is not an integer.");

    println!("Enter a value for b: ");
    io::stdin().read_line(&mut b).expect("Your input is not a valid string.");
    let b:f32 = b.trim().parse().expect("Your input is not an integer.");

    println!("Enter a value for c: ");
    io::stdin().read_line(&mut c).expect("Your input is not a valid string.");
    let c:f32 = c.trim().parse().expect("Your input is not an integer.");

    let r:f32 = (b * b) - (4.0 * a * c);

    let p:f32 = ((-b) + (r.sqrt())) / 2.0 * a;

    let q:f32 = ((-b) - (r.sqrt())) / 2.0 * a;

    println!("The roots for this equation are {} and {}.", p, q);
}