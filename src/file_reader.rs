use crate::modal::*;
use crate::definitions::common::*;
use std::collections::HashMap;

pub fn read_csv_dict(file_path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let headers: Vec<String> = rdr.headers()?.iter().map(|h| h.to_string()).collect();
    
    let mut dict_result: HashMap<String, String> = HashMap::new();

    for record in rdr.records();

    Ok(dict_result)
}