mod modal;
mod data_reader;
mod definitions;
use definitions::common::*;
use definitions::default_ms_pricing_def::*;
use definitions::default_ms_excluding_def::*;
use modal::*;
use serde::de::value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let mut debug_mode = true;
    
    let mut event_history_file_path: String = "".to_string();
    let mut excluding_def = data_reader::read_excluding_def_from_json_str(MS_EXCLUDING_DEF_JSON_STRING)?;
    let mut pricing_defs = data_reader::read_pricing_def_from_json_str(SBM_PRICING_DEF_JSON_STRING)?;

    match args.len() {
        0..=1 => {
            eprintln!("Usage: {} <event_history_file_path> <excluding_definitions_file_path> <pricing_definitions_file_path> [--debug]", args[0]);
            std::process::exit(1);
        }
        2 => {
            event_history_file_path = args[1].clone();
        }
        3 => {
            let excluding_def_file_path = &args[2];
            excluding_def = data_reader::read_excluding_def_from_json(excluding_def_file_path)?;
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
    
    let mut app_event_list: Vec<AppEvent>;

    match data_reader::read_event_from_csv(event_history_file_path.as_str()) {
        Ok(value) => {
            app_event_list = value;
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }

    if debug_mode {

    }

    if debug_mode {
        println!("Debug Mode: Enabled");
        println!("========================");
        println!("Event History File Path: {}", event_history_file_path);
        //println!("Excluding Definition File Path: {}", args[2]);
        //println!("Pricing Definition File Path: {}", args[3]);
        println!("========================");
        println!("App Event List:\n");
        for app_event in app_event_list {
            println!("Event: {:?} \n", app_event);
        }
        println!("========================");
        println!("Excluding Definition:\n{:?} \n", excluding_def);
        println!("Pricing Definitions:\n{:?} \n", pricing_defs);
    }
    Ok(())
}
