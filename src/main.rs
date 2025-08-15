mod modal;
mod data_reader;
mod definitions;
use definitions::common::*;
use definitions::default_ms_pricing_def::*;
use definitions::default_ms_excluding_def::*;
use modal::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let mut excluding_def: ExcludingDef;
    let mut pricing_defs: PricingDefs;

    match args.len() {
        0..=1 => {
            eprintln!("Usage: {} <event_history_file_path> <excluding_definitions_file_path> <pricing_definitions_file_path>", args[0]);
            std::process::exit(1);
        }
        2 => {
            pricing_defs = data_reader::read_pricing_def_from_json_str(SBM_PRICING_DEF_JSON_STRING)?;
            excluding_def = data_reader::read_excluding_def_from_json_str(MS_EXCLUDING_DEF_JSON_STRING)?;
        }
        3 => {
            let excluding_def_file_path = &args[2];
            excluding_def = data_reader::read_excluding_def_from_json(excluding_def_file_path)?;
            pricing_defs = data_reader::read_pricing_def_from_json_str(SBM_PRICING_DEF_JSON_STRING)?;
        }
        4 => {
            let excluding_def_file_path = &args[2];
            let pricing_def_file_path = &args[3];
            excluding_def = data_reader::read_excluding_def_from_json(excluding_def_file_path)?;
            pricing_defs = data_reader::read_pricing_def_from_json(pricing_def_file_path)?;
        }
        _ => {
            eprintln!("Too many arguments provided.");
            std::process::exit(1);
        }
    }
    
    let event_history_file_path: &str = args[1].as_str();

    match data_reader::read_event_from_csv(event_history_file_path) {
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

    println!("Excluding Definition: {:?}", excluding_def);
    println!("Pricing Definitions: {:?}", pricing_defs);
    Ok(())
}
