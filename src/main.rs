#![windows_subsystem = "windows"]

mod definitions;
mod modal;
mod data_io;
mod analyzing;
mod app_egui;

use anyhow::{Result, anyhow};


fn main() -> anyhow::Result<()> {
    app_egui::run().expect("Cannot run egui app!");
    Ok(())
}
