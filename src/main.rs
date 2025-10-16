#![windows_subsystem = "windows"]

slint::include_modules!();

mod definitions;
mod modal;
mod data_io;
mod analyzing;
use anyhow::{Result, anyhow};


fn main() -> anyhow::Result<()> {
    let app = MainWindow::new().unwrap().run().unwrap();
    Ok(())
}
