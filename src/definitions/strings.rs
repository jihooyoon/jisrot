pub mod success {
    pub mod success {
        pub const SPECIFIC_DATA_WRITTEN: &str = "is written to file: ";
    }
}

pub mod data {
    pub const TOTAL_STATS: &str = "Total Stats";
    pub const MERCHANT_DATA: &str = "Merchant Data";
    pub const APP_EVENTS: &str = "App Event List";
    pub const PRICING_DEFS: &str = "Pricing definitions";
    pub const EXCLUDING_DEFS: &str = "Excluding definitions";
}

pub mod ui {
    pub const BROWSE_BTN_LBL: &str = "Browse...";
    pub const ANALYZE_BTN_LBL: &str = "Analyze!";
    
    use getset::Getters;

    #[derive(Getters, Clone)]
    #[getset(get="pub")]
    pub struct UiOption <'a> {
        pub value: &'a str,
        pub text: &'a str,
    }

    pub const EXCLUDING_DEFS_OPTION_MS: UiOption = UiOption {
        value: "sbm",
        text: "MS Barcode"
    };

    pub const PRICING_DEFS_OPTION_SBM: UiOption = UiOption {
        value: "sbm",
        text: "MS Barcode"
    };

    pub const PRICING_DEFS_OPTION_SPOP: UiOption = UiOption {
        value: "spop",
        text: "MS Order Printer"
    };

    pub const OPTION_CUSTOM: UiOption = UiOption {
        value: "custom",
        text: "Custom"
    };

    pub const EXCLUDING_DEFS_OPTION_LIST: [UiOption; 1] = [
        EXCLUDING_DEFS_OPTION_MS
    ];

    pub const PRICING_DEFS_OPTION_LIST: [UiOption; 2] = [
        PRICING_DEFS_OPTION_SBM,
        PRICING_DEFS_OPTION_SPOP
    ];
}