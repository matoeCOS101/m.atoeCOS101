use std::fs::File;
use std::io::Write;
use std::error::Error;

#[derive(Debug, Clone)]
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u16, 
}

impl Student {
    fn to_file_format(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.name,
            self.matric_number,
            self.department,
            self.level
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric_number: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric_number: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric_number: "CSC10328828".to_string(),
            department: "Computer".to_string(),
            level: 200,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric_number: "EEE11020202".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Blanca Edemoh".to_string(),
            matric_number: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    let filename = "pau_smis_students.txt";
    
    println!("# PAU-SMIS Student Details #");
    
    println!("\n| {} | {} | {} | {} |", 
        "Student Name", "Matric. Number", "Department", "Level");
    println!("|--------------|--------------|------------|------|");
    
    for student in &students {
        println!("| {} | {} | {} | {} |", 
            student.name, student.matric_number, student.department, student.level);
    }
    
    println!("\n## Saving data to {}...", filename);

    let mut file = File::create(filename)?;

    let header = "Student Name|Matric. Number|Department|Level\n";
    file.write_all(header.as_bytes())?;

    for student in &students {
        let record = student.to_file_format();
        
        file.write_all(format!("{}\n", record).as_bytes())?;
    }

    println!("Data successfully saved to {}.", filename);

    Ok(())
}