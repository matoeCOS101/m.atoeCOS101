//Rust program to detemine the annual incentive
//using the experience and age of an employee

use std::io;

fn main() {
   println!("==Employee Annual Incentive Calculator==");
  
//Experience 
println!("Are you an experienced employee? (yes/no): ");
let mut experience_input = String::new();
io::stdin()
.read_line(&mut experience_input)
.expect("No input detected");

let experience_input: bool = match experience_input.trim().to_lowercase().as_str(){
    "yes" => true,
    "no" => false,
    _=> {
    println!("Invalid input! Please enter 'yes' or 'no'.");
    return; 
 }
};
  if experience_input == false {
    println!("Your incentive is N100_000 per month");
 } else if experience_input == true {
    println!("How old are you?");
    let mut age = String::new();
    io::stdin()
    .read_line(&mut age)
    .expect("No input detected");

  if age >= 40.to_string() {
    println!("Your incentive is N1_560_000 per month");
  }
  if age >= 30.to_string() && age < 40.to_string() {
    println!("Your incentive is N1_480_000 per month");
  }
  if age < 28.to_string() && age < 30.to_string() {
    println!("Your incentive is N1_300_000 per month");
  }
 }
}

