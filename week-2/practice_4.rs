fn main(){
	Let p:f64 = 1000.0;
	Let r:f64 = 1.0;
	Let t:f64 = 2.0;

	// simple interest
	let a = p * ( 1.0 + (r / 100.0)) * t;
	println!("Amount is {}", a);
	let si = a - p;
	println!("Simple Interest is {}", si);
}