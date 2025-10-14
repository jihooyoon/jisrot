#![windows_subsystem = "windows"]

mod definitions;
mod modal;
mod data_io;
mod analyzing;
mod egui_gui;

use egui_gui::*;

use eframe::egui;
use anyhow::{Result, anyhow};


fn main() -> anyhow::Result<()> {
    let egui_gui_app = QuickGUIApp::default();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([300.0, 300.0])
        .with_title("Jisrot - Tiny Shopify Event History Anal Tool"),
        ..Default::default()
    };

    eframe::run_native(
        "Jisrot - Tiny Shopify Event History Anal Tool",
        native_options,
        Box::new(|_| Ok(Box::new(egui_gui_app))),
    ).map_err(|e| anyhow!("Failed to start GUI: {}", e))?;

    Ok(())
}
