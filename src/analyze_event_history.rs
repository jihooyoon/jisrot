use crate::modal::*;
use crate::definitions::common::*;
use regex::Regex;

pub fn build_merchant_data_and_count_basic_stats (
    app_event_list: &Vec<AppEvent>,
    pricing_defs: &PricingDefs,
    excluding_def: &ExcludingDef
) -> (TotalStats, MerchantDataList) {
    let mut merchant_data_list:MerchantDataList = MerchantDataList::new();
    let mut total_stats:TotalStats = TotalStats::new(pricing_defs);

    for event in app_event_list {
        
        //Excluding check
        let mut re = Regex::new(excluding_def.excluding_pattern()).unwrap();
        if re.is_match(event.shop_email().as_str()) { // !!! HARD CODED EXCLUDING FIELD
            continue;
        }

        let mut current_merchant_data = if merchant_data_list.merchants().contains_key(event.shop_domain().as_str()) {
            merchant_data_list.merchants().get(event.shop_domain().as_str()).unwrap().clone()
        } else {
            MerchantData::new(&event.shop_domain(), pricing_defs.one_times())
        };

        //Count Install, Uninstall, Store closed
        re = Regex::new(INSTALLED_STRING).unwrap();
        if re.is_match(event.event().as_str()) {
            total_stats.increase_installed_count(1);
            current_merchant_data.increase_installed_count(1);
            current_merchant_data.push_installing_event(event);
            merchant_data_list.update_merchant(current_merchant_data);
            continue;
        }

        re = Regex::new(UNINSTALLED_STRING).unwrap();
        if re.is_match(event.event().as_str()) {
            total_stats.increase_uninstalled_count(1);
            current_merchant_data.increase_uninstalled_count(1);
            current_merchant_data.push_installing_event(event);
            merchant_data_list.update_merchant(current_merchant_data);
            continue;
        }

        re = Regex::new(STORE_CLOSED_STRING).unwrap();
        if re.is_match(event.event().as_str()) {
            total_stats.increase_store_closed_count(1);
            current_merchant_data.increase_store_closed_count(1);
            current_merchant_data.push_installing_event(event);
            merchant_data_list.update_merchant(current_merchant_data);
            continue;
        }

        
    }

    (total_stats, merchant_data_list)
}