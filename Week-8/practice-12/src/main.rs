fn main() {

    let mut colours = ["Red", "Green", "Yellow", "White"];
    println!("\nOriginal array = {:?}", colours);

    let sliced_colours = &mut colours [1..3];
    println!("First slice = {:?}", sliced_colours);

    sliced_colours [1] = "Purple";
    println!("Changed slice = {:?}", sliced_colours);

}