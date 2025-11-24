use std::io;

fn main() {
    println!("AN APS level checker");

    // APS data: (APS Level, Role, MinYears, MaxYears)
    let aps_data = vec![
        ("APS 1-2", "Intern", 0, 2),
        ("APS 1-2", "Paralegal", 0, 2),
        ("APS 1-2", "Placement", 0, 2),

        ("APS 3-5", "Administrator", 3, 5),
        ("APS 3-5", "Research Assistant", 3, 5),
        ("APS 3-5", "Junior Associate", 3, 5),
        ("APS 3-5", "Classroom Teacher", 3, 5),

        ("APS 5-8", "Senior Administrator", 5, 8),
        ("APS 5-8", "PhD Candidate", 5, 8),
        ("APS 5-8", "Associate", 5, 8),
        ("APS 5-8", "Snr Teacher", 5, 8),

        ("EL1 8-10", "Office Manager", 8, 10),
        ("EL1 8-10", "Post-Doc Researcher", 8, 10),
         ("EL1 8-10", "Senior Associate 1-2", 8, 10),
        ("EL1 8-10", "Leading Teacher", 8, 10),

        ("EL2 10-13", "Director", 10, 13),
        ("EL2 10-13", "Senior Lecturer", 10, 13),
        ("EL2 10-13", "Senior Associate 3-4", 10, 13),
        ("EL2 10-13", "Deputy Principal", 10, 13),

        ("SES", "CEO", 15, 50),
        ("SES", "Dean", 15, 50),
        ("SES", "Partner", 15, 50),
        ("SES", "Principal", 15, 50),
    ];

    // use standard input output library to input roles and years
    
    let mut role = String::new();
    let mut years_input = String::new();

    println!("Enter staff role:");
    io::stdin().read_line(&mut role).expect("Failed to read input");
    let role = role.trim();

    
    println!("Enter years of experience:");
    io::stdin().read_line(&mut years_input).expect("Failed to read input");
    let years: i32 = match years_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number. Using 0 years.");
            0
              }
    };

    //  FIND APS LEVEL 
    let result = get_aps_level(role, years, &aps_data);

    if result.len() > 0 {
        println!("APS Level: {}", result);
    } else {
        println!("No matching APS level found.");
    }
}

fn get_aps_level(role: &str, years: i32,  aps_data: &Vec<(&str, &str, i32, i32)>) -> String {
    for (aps, job_role, min_y, max_y) in aps_data {
        // Basic comparison: exact string match
        if role == *job_role {
            if years >= *min_y && years <= *max_y {
                return aps.to_string();
            }
        }
    }
    String::new() // empty string -> not found
}



