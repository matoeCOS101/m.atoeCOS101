/* Rust program that converts temperature from Celsius to Fahrenheit and Kelvin */

use std::io;

fn main() {
   
   //input temperature in celsius
   println!("\nPlease Input Temperature in Celsius");
   let  mut celc = String::new();
   io::stdin()
   .read_line(&mut celc)
   .expect("Please input temperature");
   let celc:f64 = celc.trim().parse().expect("Input not valid");

   //Temperature in Fahrenheit
   let fahr:f64 = ( 9.0 / 5.0 ) * ( celc + 32.0 );

   //Temperature in Kelvin
   let kel:f64 = celc + 273.15 ;

   println!("The temperature is : celcius = {}, fahrenheit = {}, kelvin = {}", celc, fahr, kel );

   if celc < 0.0 {

    println!("Freezing point range");
   
   } else if celc >= 0.0 && celc <= 30.0 {
   
    println!("Normal range");
   
   } else if celc > 30.0 {

    println!("Hot temperature range");
  
   } else {

     println!("Invalid input");
   
   }
}
