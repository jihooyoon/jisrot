use std::path::PathBuf;

use eframe::{egui::{CentralPanel, ComboBox, Ui, ViewportBuilder}, get_value, icon_data, run_native, set_value, App, CreationContext, NativeOptions, Storage, APP_KEY};
use serde::{self, Deserialize, Serialize};

use crate::definitions::strings::ui::*;

#[derive(Serialize, Deserialize)]
struct QuickGUIApp {
    debug_mode: bool,
    case_sensitive_regex: bool,
    selected_pricing_defs_option: String,
    selected_excluding_defs_option: String
}

impl Default for QuickGUIApp {
    fn default() -> Self {
        Self {
            debug_mode: false,
            case_sensitive_regex: false,
            selected_pricing_defs_option: PRICING_DEFS_OPTION_SBM.value().to_string(),
            selected_excluding_defs_option: EXCLUDING_DEFS_OPTION_MS.value().to_string()
        }
    }
}

impl QuickGUIApp {

    ///Called one before first frame
    fn new(cc: &CreationContext<'_>) -> Self {
        //Place for customizing look and feel
        //Using 'cc.egui_ctx.set_visuals' and 'cc.egui_ctx.set_fonts'
        

        //Load previous app state (if any)
        //Must enable 'persistence' feature to work
        if let Some(storage) = cc.storage {
            get_value(storage, APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
    
}

impl App for QuickGUIApp {

    //Called to save state before shutdown
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self);
    }

    //Called each time UI needs repainting
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui|{

        });
    }


}

fn selector_with_file_support(ui: &mut Ui, label: &str, option_list: &Vec<UiOption>, selected_slot: &mut String, file_slot: &mut Option<PathBuf>) {
    ui.vertical(|ui|{
        ui.label(label);
        ui.horizontal(|ui| {
            ComboBox::from_label(label)
                .selected_text(selected_slot.clone())
                .show_ui(ui, |ui| {
                    for option in option_list {
                        ui.selectable_value(selected_slot, option.value().to_string(), option.text().to_string());
                    }
                    ui.selectable_value(selected_slot, OPTION_CUSTOM.value().to_string(), OPTION_CUSTOM.text().to_string());
                });
        });
        if let Some(f) = file_slot {
            ui.label(f.file_name().unwrap().display().to_string());
        }
    });
}

pub fn run() -> eframe::Result<()> {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 300.0])
            /*.with_icon( ///Load app icon
                icon_data::from_png_bytes(&include_bytes!("../res/icons/icons-256.png")[..])
                    .expect("Failed to load icon")
            )*/,
        ..Default::default()
    };

    run_native(
        "Jisrot - Shopify Events Anal", 
        native_options, 
        Box::new(|cc| Ok(Box::new(QuickGUIApp::new(cc)))))
}