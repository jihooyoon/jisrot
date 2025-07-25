mod modal;
mod definitions;
use modal::*;
use definitions::common::*;

fn read_csv_raw(file_path: &str) -> Result<Vec<AppEvent>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut records = Vec::new();
    
    for result in rdr.records() {
        let record = result?;
        records.push(record.iter().map(|s| s.to_string()).collect());
    }
    
    Ok(records)
}