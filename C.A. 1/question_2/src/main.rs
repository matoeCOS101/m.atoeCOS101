// Employee Payroll Calculator

use std::io;

fn main() {
    loop{

    //input name
    println!("\nPlease input your fullname");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read input");
    println!("Your name is {}", name);
   
   if name == "no"{
    break;
}
    //input hours worked
    println!("\nPlease input the number of hours you work");
    let mut h = String::new();
    io::stdin()
    .read_line(&mut h)
    .expect("Failed to read input");
    let h:i64 = h.trim().parse().expect("Please enter a valid input");

    //Gross salary
    
    if h <= 40 {
        
        let g = 3000 * h ;
        println!("Your gross salary is N{} ", g );
        //net pay after tax
    if g > 100_000 {
        
        let n = g - 2000 ;
        println!("Your net pay is N{}", n);}
        
    } else if h > 40 {

     let g = 4500 * h ;
     println!("Your gross salary is N{} ", g );
   //net pay after tax
    if g > 100_000 {
        
        let n = g - 2000 ;
        println!("Your net pay is N{}", n);}
    }
    
    }


}

