//Using the 'iter{}' function.

fn main() {

    let arr:[i32; 4] = [10, 20, 30, 40];
        println!("Array is {:?}.", arr);
        println!("Array size is {}.", arr.len());

    for val in arr.iter() {
        println!("Value is {}.", val);
    }

}