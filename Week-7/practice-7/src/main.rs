fn main() {
    
    let arr1:[i32; 4] = [10, 20, 30, 40];
    println!("\nArray with data types.");
    println!("Array is {:?}.", arr1);
    println!("Array size is {}.", arr1.len());

    let arr2:[f32; 6] = [10.4, 20.7, 30.4, 40.9, 51.2, 72.2];
    println!("\nArray without data type.");
    println!("Array is {:?}.", arr2);
    println!("Array size is {}.", arr2.len());

    let arr3:[i32; 8] = [-1;8];
    println!("\nArray with default values.");
    println!("Array is {:?}.", arr3);
    println!("Array size is {}.", arr3.len());

}