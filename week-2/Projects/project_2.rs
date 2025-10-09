fn main(){
	let t:f64 = 450_000.0; // Toshiba;
	let m:f64 = 1_500_000.0; //Mac;
	let h:f64 = 750_000.0; //HP;
	let d:f64 = 2_850_000.0; //Dell;
	let a:f64 = 250_000.0; //Acer;
	let z:f64 = 10.0; //total number of sales;

	//Sum
	let s = (t*2.0) + m + (h*3.0) + (d*3.0) + a;
	println!("Sum of the sales record is {}", s);
	//Average
	let x = s / z;
	println!("The average sales record is {}", x);
}