use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let categories = vec![
        ("Lager", vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"]),
        ("Stout", vec!["Legend", "Turbo King", "Williams"]),
        ("Non-Alcoholic", vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"]),
    ];

    let mut file = File::create("drinks.txt")?;

    for (category, drinks) in categories {
        // Write category name
        file.write_all(format!("{}:\n", category).as_bytes())?;

        // Write each drink
        for drink in drinks {
            file.write_all(format!("- {}\n", drink).as_bytes())?;
        }

        file.write_all(b"\n")?; // blank line
    }

    println!("File created successfully.");
    Ok(())
}