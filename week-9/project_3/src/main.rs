use std::fs::File;
use std::io::Write;
use std::error::Error;

#[derive(Debug, Clone)]
struct Ministers{
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

impl Ministers {
    fn to_file_format(&self) -> String {
        format!(
            "{}|{}|{}",
            self.name,
            self.ministry,
            self.geopolitical_zone
            )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
 let ministers = vec![
   Ministers{
    name: "Aigbogun Alamba Daudu".to_string(),
    ministry: "Internal Affairs".to_string(),
    geopolitical_zone: "South West".to_string(),
   },
   Ministers{
    name: "Murtala Afeez Bendu".to_string(),
    ministry: "Justice".to_string(),
    geopolitical_zone: "North East".to_string(),
   },
   Ministers{
    name: "Okorocha Calistus Ogbona".to_string(),
    ministry: "Defense".to_string(),
    geopolitical_zone: "South South".to_string(),
   },
   Ministers{
    name: "Adewale Jimoh Akanbi".to_string(),
    ministry: "Power & Steel".to_string(),
    geopolitical_zone: "South West".to_string(),
   },
   Ministers{
    name: "Osazuwa Faith Etieye".to_string(),
    ministry: "Petroleum".to_string(),
    geopolitical_zone: "South East".to_string(),
   },
 ];

 let filename = "convicted_ministers.txt";

 println!("# Convicted Ministers From All Geopolitical Zones #");

 println!("\n| {} | {} | {} |",
    "Name of Commisioner", "Ministry", "Geopolitical zone");
 println!("|---------------------|----------|-------------------|");

 for ministers in &ministers {
    println!("| {} | {} | {} |", ministers.name, ministers.ministry, ministers.geopolitical_zone);
 }

 println!("\n## Saving data to {}...", filename);

 let mut file = File::create(filename)?;

 let header = "Name of Commisioner|Ministry|Geopolitical zone\n";
    file.write_all(header.as_bytes())?;

    for ministers in &ministers {
        let record = ministers.to_file_format();
        
        file.write_all(format!("{}\n", record).as_bytes())?;
    }

    println!("Data successfully saved to {}.", filename);

    Ok(())
}