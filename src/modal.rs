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
    one_time_events: Vec<HashMap<String, String>>,
    installed_status: String,
    subscription_status: String,
}

struct MerchantDataList {
    start_time: String,
    end_time: String,
    merchants: HashMap<String, MerchantData>,
}