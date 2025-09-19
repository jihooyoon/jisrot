use crate::modal::*;
use crate::definitions::common::*;
use std::collections::HashMap;

pub fn read_events_from_csv(file_path: &str) -> Result<Vec<AppEvent>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let headers: Vec<String> = rdr.headers()?.iter().map(|h| h.to_string()).collect();
    
    let mut app_event_list: Vec<AppEvent> = Vec::new();

    for record in rdr.records(){
        let record_hash: HashMap<String, String> = record?.iter().zip(headers.iter())
            .map(|(value, header)|(header.to_string(), value.to_string()))
            .collect();
        
        match AppEvent::from_hashmap(&record_hash) {
            Ok(app_event) => app_event_list.push(app_event),
            Err(e) => println!("Error parsing record: {}", e),
        }
    }

    Ok(app_event_list)
}

pub fn read_pricing_def_from_json(file_path: &str) -> Result<PricingDefs, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    
    let result = serde_json::from_reader(reader)?;
    Ok(result)
}

pub fn read_pricing_def_from_json_str(json_str: &str) -> Result<PricingDefs, serde_json::Error> {
    Ok(serde_json::from_str(json_str)?)
}

pub fn read_excluding_def_from_json(file_path: &str) -> Result<ExcludingDef, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    
    let result = serde_json::from_reader(reader)?;
    Ok(result)
}

pub fn read_excluding_def_from_json_str(json_str: &str) -> Result<ExcludingDef, serde_json::Error> {
    Ok(serde_json::from_str(json_str)?)
}