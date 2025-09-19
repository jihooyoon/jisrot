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
        if re.is_match(event.shop_email.as_str()) { //!!! HARD CODED EXCLUDING FIELD
            continue;
        }

        let current_merchant_data: MerchantData = MerchantData::new(&event.shop_domain, one_time_packs);

        //Count Install, Uninstall, Store closed
        re = Regex::new(INSTALLED_STRING).unwrap();
        if re.is_match(event.event.as_str()) {
            total_stats.installed_count += 1;
        }

    }

    (total_stats, merchant_data_list)
}