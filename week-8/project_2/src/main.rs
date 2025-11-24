use std::io;

fn main() {
    // Vector to store developers as tuples: (name, years_of_experience)
    let mut developers: Vec<(String, u32)> = Vec::new();

    println!("Enter number of developers interviewed:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read number");
    let num: usize = input.trim().parse().expect("Please enter a valid number");

    for i in 0..num {
        println!("\nEnter name of developer {}:", i + 1);
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read name");

        println!("Enter years of experience for {}:", name.trim());
        let mut exp = String::new();
        io::stdin().read_line(&mut exp).expect("Failed to read experience");
        let experience: u32 = exp.trim().parse().expect("Enter a valid number");

        developers.push((name.trim().to_string(), experience));
    }

     // Assume first developer is the most experienced initially
    let mut best_name = developers[0].0.clone();
    let mut best_exp = developers[0].1;

    // Compare each developer’s experience using if/else only
    for i in 1..developers.len() {
        let current_name = &developers[i].0;
        let current_exp = developers[i].1;

        if current_exp > best_exp {
            best_exp = current_exp;
            best_name = current_name.clone();
        }
    }

    println!("\n----------------------------------");
    println!("The developer with the highest years of experience is:");
    println!("{} with {} years of experience.", best_name, best_exp);
}