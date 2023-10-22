fn main(){

	let p:f64 = 210000.00;
	let r:f64 = 5.00;
	let n:f64 = 3.00;

	let a = p * (1 - (r / 100)) ^ n;
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Compound interest is {}", ci);
	println!("Therefore, the value of the television after depreciation is {}", ci);
	
}