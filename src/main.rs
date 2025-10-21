#![windows_subsystem = "windows"]

mod analyzing;
mod app_egui;
mod data_io;
mod definitions;
mod modals;

use std::env::args;

use anyhow::{Result, anyhow};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = args().collect();

    if args[0] == "reset" {
        app_egui::run(true).expect("Cannot run egui app!");
    }
    app_egui::run(false).expect("Cannot run egui app!");
    Ok(())
}
