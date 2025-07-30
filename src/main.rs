mod modal;
mod file_reader;
mod definitions;
use definitions::common::*;
use modal::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path: &str = args[1].as_str();

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
