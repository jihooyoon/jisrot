mod modal;
mod file_reader;
mod definitions;
use std::path;

use definitions::common::*;
use modal::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    
    let file_path: &str = args[1].as_str();

    let path = path::Path::new(file_path);

    if let Some(file_ext) = path.extension().and_then(|s| s.to_str()) {
        if file_ext == "json" {
            match file_reader::read_pricing_def_from_json(file_path) {
                Ok(pricing_defs) => {
                    println!("Pricing definitions loaded successfully: {:?}", pricing_defs);
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error reading JSON file: {}", e);
                    std::process::exit(1);
                }
            }
        } 
    }

    match file_reader::read_event_from_csv(file_path) {
        Ok(app_event_list) => {
            for app_event in app_event_list {
                println!("Event: {:?}", app_event);
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}
