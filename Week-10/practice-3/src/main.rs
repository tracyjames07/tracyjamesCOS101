fn main() {

    let v = vec![40, 40, 60, 80];

    let v2 = v;
    let v2_return = display(v2);
    println!("In main {:?}", v);

}

fn display(v: Vec<i32>)->Vec<i32> {

    println!("Inside display {:?}", v);
    return v;
    
}