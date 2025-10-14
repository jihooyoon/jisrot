use eframe::egui::Options;
use eframe::{egui};
use rfd;
use std::path::PathBuf;
use std::env;
use anyhow::{anyhow, Error};

use crate::definitions::common::*;
use crate::definitions::default_ms_pricing_def::*;
use crate::definitions::default_ms_excluding_def::*;
use crate::modal::*;
use crate::data_io::*;
use crate::analyzing::*;

pub struct QuickGUIApp {
    out_total_stats_file: PathBuf,
    out_merchant_data_file: PathBuf,
    out_app_event_list_file: PathBuf,
    history_data_file: Option<PathBuf>,
    excluding_defs_file: Option<PathBuf>,
    pricing_defs_file: Option<PathBuf>,
    excluding_defs: ExcludingDef,
    pricing_defs: PricingDefs,
    app_event_list: Vec<AppEvent>,
    total_stats: TotalStats,
    merchant_data_list: MerchantDataList,
    debug_mode: bool,
}

impl Default for QuickGUIApp {
    fn default() -> Self {
        let excluding_defs: ExcludingDef = read_excluding_def_from_json_str(MS_EXCLUDING_DEF_JSON_STRING).unwrap();
        let pricing_defs: PricingDefs = read_pricing_def_from_json_str(SBM_PRICING_DEF_JSON_STRING).unwrap();
        let total_stats = TotalStats::new(&pricing_defs);
        let merchant_data_list = MerchantDataList::new();

        Self {
            out_total_stats_file: PathBuf::from("./"),
            out_merchant_data_file: PathBuf::from("./"),
            out_app_event_list_file: PathBuf::from("./"),
            history_data_file: None,
            excluding_defs_file: None,
            pricing_defs_file: None,
            excluding_defs: excluding_defs,
            pricing_defs: pricing_defs,
            app_event_list: Vec::new(),
            total_stats: total_stats,
            merchant_data_list: merchant_data_list,
            debug_mode: false,
        }
    }
}

impl eframe::App for QuickGUIApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Select files, then click Analyze");
            
            ui.separator();

            ui.vertical(|ui| {
                ui.add_space(8.0);
                ui.spacing_mut().item_spacing.y = 16.0; // Increase vertical spacing between elements
                file_picker_button(ui, "history data", "No file selected!", &mut self.history_data_file);
                file_picker_button(ui, "excluding definitions", "Using default (Magestore)", &mut self.excluding_defs_file);
                file_picker_button(ui, "pricing definitions","Using default (MS Barcode)", &mut self.pricing_defs_file);
                if ui.button("Analyze").clicked() {
                    if let Some(path) = &self.excluding_defs_file {
                        match read_excluding_def_from_json(path.to_str().unwrap()) {
                            Ok(def) => self.excluding_defs = def,
                            Err(e) => {
                                rfd::MessageDialog::new()
                                    .set_title("File Reading Error")
                                    .set_description(format!("Error reading excluding definitions file: {}", e))
                                    .set_level(rfd::MessageLevel::Error)
                                    .show();
                            }
                        }
                    }

                    if let Some(path) =  {&self.pricing_defs_file} {
                        match read_pricing_def_from_json(path.to_str().unwrap()) {
                            Ok(def) => self.pricing_defs = def,
                            Err(e) => {
                                rfd::MessageDialog::new()
                                    .set_title("File Reading Error")
                                    .set_description(format!("Error reading pricing definitions file: {}", e))
                                    .set_level(rfd::MessageLevel::Error)
                                    .show();
                            },
                        }                        
                    }

                    match &self.history_data_file {
                        Some(path) => {
                            match read_events_from_csv(path.to_str().unwrap()) {
                                Ok(events) => {
                                    self.app_event_list = events;
                                    
                                    (self.total_stats, self.merchant_data_list) = build_merchant_data_and_count_basic_stats(
                                        &self.app_event_list, 
                                        &self.pricing_defs, 
                                        &self.excluding_defs);
                                    
                                    process_merchant_data_and_count_final_stats(
                                        &mut self.total_stats, 
                                        &mut self.merchant_data_list, 
                                        &self.pricing_defs);

                                    
                                    match env::current_dir() {
                                        Ok(path) => {
                                            self.out_total_stats_file = path.clone();
                                            self.out_total_stats_file.push("output");
                                            self.out_total_stats_file.push(format!("{}_{}_total_stats.json",
                                                                                    self.total_stats.start_time_str(),
                                                                                    self.total_stats.end_time_str()));
                                            self.out_merchant_data_file = path.clone();
                                            self.out_merchant_data_file.push("output");
                                            self.out_merchant_data_file.push(format!("{}_{}_merchant_data.json", 
                                                                                    self.total_stats.start_time_str(), 
                                                                                    self.total_stats.end_time_str()));
                                            self.out_app_event_list_file = path.clone();
                                            self.out_app_event_list_file.push("output");
                                            self.out_app_event_list_file.push(format!("{}_{}_app_event_list.json", 
                                                                                    self.total_stats.start_time_str(), 
                                                                                    self.total_stats.end_time_str()));
                                            
                                            if let Err(e) = write_total_stats_to_json(&self.out_total_stats_file, &self.total_stats) {
                                                rfd::MessageDialog::new()
                                                    .set_title("File Writing Error")
                                                    .set_description(format!("Error writing total stats data: {}", e))
                                                    .set_level(rfd::MessageLevel::Error)
                                                    .show();
                                            } else {
                                                if self.debug_mode {
                                                    match (write_merchant_data_to_json(&self.out_merchant_data_file, &self.merchant_data_list), 
                                                        write_app_event_list_to_json(&self.out_app_event_list_file, &self.app_event_list)) {
                                                        (Ok(()), Ok(())) => {
                                                            rfd::MessageDialog::new()
                                                                .set_title("Output Data Written")
                                                                .set_description(format!("Written total stats data to: {}\nWritten merchant data to: {}\nWritten app event list to: {}", 
                                                                                                self.out_total_stats_file.display(), 
                                                                                                self.out_merchant_data_file.display(),
                                                                                                self.out_app_event_list_file.display()))
                                                                .set_level(rfd::MessageLevel::Info)
                                                                .show();
                                                        },
                                                        (Err(e), _) => {
                                                            rfd::MessageDialog::new()
                                                                .set_title("File Writing Error")
                                                                .set_description(format!("Error writing merchant data: {}", e))
                                                                .set_level(rfd::MessageLevel::Error)
                                                                .show();
                                                        }
                                                        (_, Err(e)) => {
                                                            rfd::MessageDialog::new()
                                                                .set_title("File Writing Error")
                                                                .set_description(format!("Error writing app event data: {}", e))
                                                                .set_level(rfd::MessageLevel::Error)
                                                                .show();
                                                        }
                                                    }
                        
                                                } else {
                                                    rfd::MessageDialog::new()
                                                        .set_title("Output Data Written")
                                                        .set_description(format!("Written total stats data to: {}", 
                                                                                        self.out_total_stats_file.display()))
                                                        .set_level(rfd::MessageLevel::Info)
                                                        .show();
                                                }
                                            }
                    
                                            
                                            
                                        },
                                        Err(e) => {
                                            rfd::MessageDialog::new()
                                                .set_title("File Reading Error")
                                                .set_description(format!("Error finding working dir: {}", e))
                                                .set_level(rfd::MessageLevel::Error)
                                                .show();
                                        }
                                    }
                                }
                                Err(e) => {
                                    rfd::MessageDialog::new()
                                                .set_title("File Reading Error")
                                                .set_description(format!("Error reading history data file: {}", e))
                                                .set_level(rfd::MessageLevel::Error)
                                                .show();
                                }
                            }
                        },
                        None => {
                            rfd::MessageDialog::new()
                                                .set_title("File Reading Error")
                                                .set_description("No history file!")
                                                .set_level(rfd::MessageLevel::Error)
                                                .show();
                        }
                    }
                }
                // ui.add_space(8.0);
            });
            
            ui.separator();

            ui.checkbox(&mut self.debug_mode, "Enable debug mode");
        });
    }
}

fn file_picker_button(ui: &mut egui::Ui, file_content: &str, no_file_label: &str, file_slot: &mut Option<PathBuf>) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0; // Space between button and label
        if ui.button(format!("Select {} file...", file_content)).clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                *file_slot = Some(path);
            }
        }

        match file_slot {
            Some(path) => ui.label(format!("Selected: {}", path.display())),
            None => ui.label(no_file_label)
        }
    });
}