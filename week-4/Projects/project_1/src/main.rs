//Rust program to determine the roots of a quadratic equation
use std::io;

fn main() {
   println!("Enter the value for a: ");  
   let mut input1 = String::new();
   io::stdin().read_line(&mut input1).expect("Not a valid input");
   let a:f64 = input1.trim().parse().expect("Failed to input");

    println!("Enter the value for b: ");  
   let mut input2 = String::new();
   io::stdin().read_line(&mut input2).expect("Not a valid input");
   let b:f64 = input2.trim().parse().expect("Failed to input");

    println!("Enter the value for c: ");  
   let mut input3 = String::new();
   io::stdin().read_line(&mut input3).expect("Not a valid input");
   let c:f64 = input3.trim().parse().expect("Failed to input");

   //Discriminant
   let d = (b * b) - (4.0 * a * c);

   if d > 0.0 {
    let root1 = (-b + d.sqrt()) / (2.0 * a);
    let root2 = (-b + d.sqrt()) / (2.0 * a);
    println!("The equation has two distinct real roots:  x₁ = {}, x₂ = {} ", root1, root2);
   } else if let _d = 0.0 {
       let root = -b / (2.0 * a);
       println!("The equation has one real root repeated twice: x = {}", root);
   }else {
    let real_part = -b / (2.0 * a);
    let imaginary_part = (-d).sqrt() / (2.0 * a);
    println!(
         "Complex roots: x₁ = {} + {}i, x₂ = {} - {}i", 
         real_part, imaginary_part, real_part, imaginary_part
        );
   }
}
