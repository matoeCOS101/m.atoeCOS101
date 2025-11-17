// calculating areas and volumes of different shapes
use std::io;
 // functions for each shape's formula
fn trapezium() {
    let mut height_input = String::new();
    let mut base1_input = String::new();
    let mut base2_input = String::new();

    println!("\nEnter the height: ");
    io::stdin().read_line(&mut height_input).expect("Failed to read input");
    let h:f32 = height_input.trim().parse().expect("Not a valid Number");

    println!("\nEnter the first base: ");
    io::stdin().read_line(&mut base1_input).expect("Failed to read input");
    let b1:f32 = base1_input.trim().parse().expect("Not a valid Number");

    println!("\nEnter the second base:");
    io::stdin().read_line(&mut base2_input).expect("Failed to read input");
    let b2:f32 = base2_input.trim().parse().expect("Not a Valid Number");

    let trapezium = (h / 2.0) * (b1 + b2);
    println!("\nThe Area of the Trapezium is {}",trapezium);
}

fn rhombus(){
    let mut diagonal1_input = String::new();
    let mut diagonal2_input = String::new();

    println!("\nEnter the first diagonal: ");
    io::stdin().read_line(&mut diagonal1_input).expect("Failed to read input");
    let d1:f32 = diagonal1_input.trim().parse().expect("Not a valid Number");

    println!("\nEnter the second diagonal: ");
    io::stdin().read_line(&mut diagonal2_input).expect("Failed to read input");
    let d2:f32 = diagonal2_input.trim().parse().expect("Not a valid Number");

    let rhombus = ( 1.0 / 2.0) * d1 * d2;
    println!("\nThe Area of the Rhombus is {}",rhombus);
}

fn parallelogram(){
    let mut base_input = String::new();
    let mut altitude_input = String::new();

    println!("\nEnter the base: ");
    io::stdin().read_line(&mut base_input).expect("Failed to read input");
    let b:f32 = base_input.trim().parse().expect("Not a valid Number");

    println!("\nEnter the altitude: ");
    io::stdin().read_line(&mut altitude_input).expect("Failed to read input");
    let a:f32 = altitude_input.trim().parse().expect("Not a valid Number");

    let parallelogram = b * a;
    println!("\nThe Area of the Parallelogram is {}",parallelogram);
}

fn cube(){
    let mut side_input = String::new();

    println!("\nEnter the length of the side: ");
    io::stdin().read_line(&mut side_input).expect("Failed to read input");
    let s:f32 = side_input.trim().parse().expect("Not a valid Number");

    let cube = 6.0 * (s.powf(2.0));
    println!("\nThe Area of the cube is {}",cube);
}

fn cylinder(){
    let mut radius_input = String::new();
    let mut height_input = String::new();

    println!("\nEnter the radius: ");
    io::stdin().read_line(&mut radius_input).expect("Failed to read input");
    let r:f32 = radius_input.trim().parse().expect("Not a valid Number");

    println!("\nEnter the height: ");
    io::stdin().read_line(&mut height_input).expect("Failed to read input");
    let h:f32 = height_input.trim().parse().expect("Not a valid Number");

    let cylinder = 3.142 * (r.powf(2.0)) * h;
    println!("\nThe Volume of the Cylinder is {}",cylinder);
}

//invoke the main function for the calculator
fn main () {
    println!("\n Welcome to the calculator!! :)
              \nThe following shapes are the only shapes supported by the calculator
              \n1. Trapezium
              \n2. Rhombus
              \n3. Parallelogram
              \n4. Cube
              \n5. Cylinder");

    let mut input = String::new();

    println!("\nChoose between the following 1/2/3/4/5:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input:u32 = input.trim().parse().expect("Not a valid Number");

    //call each function

    if input == 1 {
    trapezium();
    } else if input == 2 {
    rhombus();
    } else if input == 3 {
    parallelogram();
    } else if input == 4 {
    cube();
    } else if input == 5 {
    cylinder();
    } else if input >= 6 {
        println!("\nOverly complex, cannot be done by the calculator");
    }

}
