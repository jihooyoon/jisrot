use std::path::PathBuf;

use eframe::{
    egui::{Button, CentralPanel, ComboBox, Ui, ViewportBuilder},
    epaint::tessellator::Path,
    get_value, icon_data, run_native, set_value, App, CreationContext, NativeOptions, Storage,
    APP_KEY,
};
use rfd::FileDialog;
use serde::{self, Deserialize, Serialize};

use crate::definitions::strings::{message, ui::*};
use crate::modals::ui_modal::*;
use crate::{analyzing::analyze_from_gui, definitions::strings::data::*};

#[derive(Serialize, Deserialize)]
struct QuickGUIApp {
    debug_mode: bool,
    case_sensitive_regex: bool,

    event_history_file_list: Option<Vec<PathBuf>>,

    selected_pricing_defs_option: UiOption,
    pricing_defs_file: Option<PathBuf>,

    selected_excluding_defs_option: UiOption,
    excluding_defs_file: Option<PathBuf>,
}

impl Default for QuickGUIApp {
    fn default() -> Self {
        Self {
            debug_mode: false,
            case_sensitive_regex: false,
            event_history_file_list: None,
            selected_pricing_defs_option: PRICING_DEFS_OPTION_SBM,
            pricing_defs_file: None,
            selected_excluding_defs_option: EXCLUDING_DEFS_OPTION_MS,
            excluding_defs_file: None,
        }
    }
}

impl QuickGUIApp {
    ///Called one before first frame
    fn new(cc: &CreationContext<'_>, reset_default: bool) -> Self {
        //Place for customizing look and feel
        //Using 'cc.egui_ctx.set_visuals' and 'cc.egui_ctx.set_fonts'

        //Load previous app state (if any)
        //Must enable 'persistence' feature to work
        if let Some(storage) = cc.storage {
            if !reset_default {
                return get_value(storage, APP_KEY).unwrap_or_default();
            }
        }
        Default::default()
    }
}

impl App for QuickGUIApp {
    //Called to save state before shutdown
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self);
    }

    //Called each time UI needs repainting
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let text_pricing = self.selected_pricing_defs_option.clone();
                let text_excluding = self.selected_excluding_defs_option.clone();
                selector_with_file_support(
                    ui,
                    PRICING_DEFS,
                    SELECTOR_PRICING_DEFS_ID,
                    &PRICING_DEFS_OPTION_LIST.to_vec(),
                    &mut self.selected_pricing_defs_option,
                    &mut self.pricing_defs_file,
                );

                selector_with_file_support(
                    ui,
                    EXCLUDING_DEFS,
                    SELECTOR_EXCLUDING_DEFS_ID,
                    &EXCLUDING_DEFS_OPTION_LIST.to_vec(),
                    &mut self.selected_excluding_defs_option,
                    &mut self.excluding_defs_file,
                );
            });
            ui.separator();

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if ui.button(BTN_EVENT_FILE_PICKER_LBL).clicked() {
                    self.event_history_file_list = FileDialog::new()
                        .add_filter("csv", &["csv", "CSV"])
                        .pick_files();
                }

                if ui.button(BTN_ANALYZE_LBL).clicked() {
                    match analyze_from_gui(
                        &self.event_history_file_list,
                        &self.selected_pricing_defs_option,
                        &self.selected_excluding_defs_option,
                        &self.pricing_defs_file,
                        &self.excluding_defs_file,
                        self.debug_mode,
                        self.case_sensitive_regex,
                    ) {
                        Ok(m) => {
                            rfd::MessageDialog::new()
                                .set_description(m)
                                .set_level(rfd::MessageLevel::Info)
                                .show();
                        }
                        Err(e) => {
                            rfd::MessageDialog::new()
                                .set_description(format!("{:?}", e))
                                .set_level(rfd::MessageLevel::Error)
                                .show();
                        }
                    }
                }
            });

            ui.add_space(4.0);

            ui.separator();
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.debug_mode, CHECKBOX_DEBUG_MODE_LBL);
                ui.checkbox(
                    &mut self.case_sensitive_regex,
                    CHECKBOX_CASE_SENSITIVE_REGEX_LBL,
                );
            });
        });
    }
}

fn selector_with_file_support(
    ui: &mut Ui,
    label: &str,
    selector_id: &str,
    option_list: &Vec<UiOption>,
    selected_option: &mut UiOption,
    file_slot: &mut Option<PathBuf>,
) {
    let mut current_value: String = selected_option.value().to_string();

    ui.vertical(|ui| {
        ui.label(label);
        ui.horizontal(|ui| {
            ComboBox::from_id_salt(selector_id)
                .selected_text(selected_option.text().to_string())
                .show_ui(ui, |ui| {
                    for option in option_list {
                        ui.selectable_value(
                            &mut current_value,
                            option.value().to_string(),
                            option.text().to_string(),
                        );
                    }
                    ui.selectable_value(
                        &mut current_value,
                        OPTION_CUSTOM.value().to_string(),
                        OPTION_CUSTOM.text().to_string(),
                    );
                });

            if let Some(o) = option_list.iter().find(|o| o.value == current_value) {
                *selected_option = o.clone();
                ui.add_enabled(false, Button::new(BTN_BROWSE_LBL));
            } else {
                *selected_option = OPTION_CUSTOM.clone();
                if ui.button(BTN_BROWSE_LBL).clicked() {
                    *file_slot = FileDialog::new()
                        .add_filter("json", &["json", "JSON"])
                        .pick_file();
                }
            }
        });

        if let Some(f) = file_slot {
            ui.label(f.file_name().unwrap().display().to_string());
        }
    });
}

pub fn run(reset_default: bool) -> eframe::Result<()> {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 300.0]), /*.with_icon( ///Load app icon
                                                      icon_data::from_png_bytes(&include_bytes!("../res/icons/icons-256.png")[..])
                                                          .expect("Failed to load icon")
                                                  )*/
        ..Default::default()
    };

    run_native(
        "Jisrot - Shopify Events Anal",
        native_options,
        Box::new(|cc| Ok(Box::new(QuickGUIApp::new(cc, reset_default)))),
    )
}
