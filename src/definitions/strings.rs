pub mod message {
    pub mod success {
        pub const SPECIFIC_DATA_WRITTEN: &str = "is written to file: ";
    }
    pub mod error {
        pub const FILE_NOT_CHOSEN: &str = "File not chosen";
    }
}

pub mod data {
    pub const KIND_PREDEFINED: &str = "Pre-defined";
    pub const KIND_CUSTOM: &str = "Custom";
    pub const TOTAL_STATS: &str = "Total Stats";
    pub const MERCHANT_DATA: &str = "Merchant Data";
    pub const APP_EVENTS: &str = "App Event List";
    pub const PRICING_DEFS: &str = "Pricing definitions";
    pub const EXCLUDING_DEFS: &str = "Excluding definitions";
}

pub mod ui {
    use std::borrow::Cow;

    use crate::{
        definitions::{
            default_ms_excluding_def::MS_EXCLUDING_DEF_JSON_STRING,
            default_ms_pricing_def::{SBM_PRICING_DEF_JSON_STRING, SPOP_PRICING_DEF_JSON_STRING},
        },
        modals::ui_modal::*,
    };

    pub const BTN_BROWSE_LBL: &str = "Browse...";
    pub const BTN_ANALYZE_LBL: &str = "Analyze!";
    pub const BTN_EVENT_FILE_PICKER_LBL: &str = "Browse event history file...";

    pub const DEFAULT_SELECTOR_TEXT: &str = "- Please select -";

    pub const SELECTOR_PRICING_DEFS_ID: &str = "selector_pricing_defs";
    pub const SELECTOR_EXCLUDING_DEFS_ID: &str = "selector_excluding_defs";

    pub const CHECKBOX_DEBUG_MODE_LBL: &str = "Debug mode";
    pub const CHECKBOX_CASE_SENSITIVE_REGEX_LBL: &str = "Case-sensitive regex";

    pub const EXCLUDING_DEFS_OPTION_MS: UiOption = UiOption {
        value: Cow::Borrowed("magestore"),
        text: Cow::Borrowed("Magestore"),
        connected_data: Some(Cow::Borrowed(MS_EXCLUDING_DEF_JSON_STRING)),
    };

    pub const PRICING_DEFS_OPTION_SBM: UiOption = UiOption {
        value: Cow::Borrowed("sbm"),
        text: Cow::Borrowed("MS Barcode"),
        connected_data: Some(Cow::Borrowed(SBM_PRICING_DEF_JSON_STRING)),
    };

    pub const PRICING_DEFS_OPTION_SPOP: UiOption = UiOption {
        value: Cow::Borrowed("spop"),
        text: Cow::Borrowed("MS Order Printer"),
        connected_data: Some(Cow::Borrowed(SPOP_PRICING_DEF_JSON_STRING)),
    };

    pub const OPTION_CUSTOM: UiOption = UiOption {
        value: Cow::Borrowed("custom"),
        text: Cow::Borrowed("Custom"),
        connected_data: None,
    };

    pub const EXCLUDING_DEFS_OPTION_LIST: [UiOption; 1] = [EXCLUDING_DEFS_OPTION_MS];

    pub const PRICING_DEFS_OPTION_LIST: [UiOption; 2] =
        [PRICING_DEFS_OPTION_SBM, PRICING_DEFS_OPTION_SPOP];
}
