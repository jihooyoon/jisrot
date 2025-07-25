use std::collections::HashMap;

struct MerchantData {
    checked: bool,
    installed_count: u32,
    uninstalled_count: u32,
    store_closed_count: u32,
    store_reopened_count: u32,
    installing_events: Vec<HashMap<String, String>>,
    subscription_activated_count: u32,
    subscription_canceled_count: u32,
    subscription_events: Vec<HashMap<String, String>>,
    one_time_count: u32,
    one_time_details: HashMap<String, String>,
    one_time_events: Vec<AppEvent>,
    installed_status: String,
    subscription_status: String,
}

struct MerchantDataList {
    start_time: String,
    end_time: String,
    merchants: HashMap<String, MerchantData>,
}

struct AppEvent {
    time: String,
    event: String,
    details: String,
    billing_on: String,
    shop_name: String,
    shop_country: String,
    shop_email: String,
    shop_domain: String,
    key: String    
}