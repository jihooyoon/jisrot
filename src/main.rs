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

    println!("{:?}", file_reader::read_csv_dict(file_path));
}
