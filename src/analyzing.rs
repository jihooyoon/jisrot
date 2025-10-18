use anyhow::Error;
use regex::Regex;
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::data_io::*;
use crate::definitions::default_ms_pricing_def::*;
use crate::definitions::default_ms_excluding_def::*;
use crate::modal::*;
use crate::definitions::common::*;
use crate::definitions::strings::*;

pub fn build_merchant_data_and_count_basic_stats (
    app_event_list: &Vec<AppEvent>,
    pricing_defs: &PricingDefs,
    excluding_def: &ExcludingDef
) -> (TotalStats, MerchantDataList) {
    let mut merchant_data_list:MerchantDataList = MerchantDataList::new();
    let mut total_stats:TotalStats = TotalStats::new(pricing_defs);

    total_stats.set_start_time(app_event_list.first().unwrap().time().clone());
    total_stats.set_end_time(app_event_list.last().unwrap().time().clone());
    merchant_data_list.set_start_time(app_event_list.first().unwrap().time().clone());
    merchant_data_list.set_end_time(app_event_list.last().unwrap().time().clone());
    
    total_stats.build_pretty_time_str();

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

        re = Regex::new(STORE_REOPENED_STRING).unwrap();
        if re.is_match(event.event().as_str()) {
            total_stats.increase_store_reopened_count(1);
            current_merchant_data.increase_store_reopened_count(1);
            current_merchant_data.push_installing_event(event);
            merchant_data_list.update_merchant(current_merchant_data);
            continue;
        }

        // Count One-Time
        if ONE_TIME_ACTIVATED_STRINGS.contains(&event.event().as_str()) {
            total_stats.increase_one_time_count(1);
            current_merchant_data.increase_one_time_count(1);
            current_merchant_data.push_one_time_event(event);

            for pack in pricing_defs.one_times() {
                re = Regex::new(pack.regex_pattern().as_str()).unwrap();
                if re.is_match(event.details().as_str()) {
                    total_stats.increase_one_time_pack_count(pack, 1).unwrap();
                    current_merchant_data.increase_one_time_pack_count(pack, 1).unwrap();
                    break;
                }
            }
            merchant_data_list.update_merchant(current_merchant_data);
            continue;
        }

        // Count Subscriptions
        if SUBSCRIPTION_ACTIVATED_STRINGS.contains(&event.event().as_str()) {
            current_merchant_data.increase_subscription_activated_count(1);
            current_merchant_data.push_subscription_event(event);
            merchant_data_list.update_merchant(current_merchant_data);
            continue;
        }
        if SUBSCRIPTION_CANCELED_STRINGS.contains(&event.event().as_str()) {
            current_merchant_data.increase_subscription_canceled_count(1);
            current_merchant_data.push_subscription_event(event);
            merchant_data_list.update_merchant(current_merchant_data);
            continue;
        }

    }

    (total_stats, merchant_data_list)
}

pub fn process_merchant_data_and_count_final_stats(
    total_stats: &mut TotalStats,
    merchant_data_list: &mut MerchantDataList,
    pricing_defs: &PricingDefs
) -> anyhow::Result<()> {

    // Calculate final stats
    total_stats.set_merchant_growth(
        *total_stats.installed_count() as i32 + *total_stats.store_reopened_count() as i32 
        - *total_stats.uninstalled_count() as i32 - *total_stats.store_closed_count() as i32);
    
    total_stats.set_total_churn_rate(
        if *total_stats.installed_count() > 0 {
            (*total_stats.uninstalled_count() as f64 / *total_stats.installed_count() as f64) * 100.0
        } else {
            0.0
        });

    
    // Process merchant data
    for merchant in &mut merchant_data_list.merchants_mut().values_mut() {
        // Updated installed status
        match *merchant.installed_count() as i32 + *merchant.store_reopened_count() as i32 
        - *merchant.uninstalled_count() as i32 - *merchant.store_closed_count() as i32 {
            delta if delta > 0 => {
                merchant.set_installed_status(INSTALLED_STRING.to_string());
            },
            delta if delta < 0 => {
                merchant.set_installed_status(UNINSTALLED_STRING.to_string());
                if merchant.installing_events().len() > 0 
                && merchant.installing_events().first().unwrap().event() == UNINSTALLED_STRING {
                    merchant.set_installed_status(UNINSTALLED_OLD_STRING.to_string());
                    total_stats.increase_old_uninstalled_count(1);
                }
            },
            _ => { 
                merchant.set_installed_status(NONE.to_string());
            }
        }

        
        // Determine final subscription status
        match *merchant.subscription_activated_count() as i32 
        - *merchant.subscription_canceled_count() as i32 {
            delta if delta > 0 => {
                merchant.set_subscription_status(SUBSCRIPTION_STATUS_ACTIVE.to_string());
                total_stats.increase_new_sub_count(1);
            },
            delta if delta < 0 => {
                merchant.set_subscription_status(SUBSCRIPTION_STATUS_CANCELED.to_string());
                total_stats.increase_canceled_sub_count(1);
            },
            _ => {
                merchant.set_subscription_status(NONE.to_string());
            }
        }

        // Determine new subscription details
        for event in merchant.clone().subscription_events().iter().rev() { // Use reverse order to get the latest activated event
            if SUBSCRIPTION_ACTIVATED_STRINGS.contains(&event.event().as_str()) {
                // Determine plan
                for plan in pricing_defs.subscriptions() {
                    let mut re = Regex::new(plan.regex_pattern().as_str()).unwrap();
                    if re.is_match(event.details().as_str()) {
                        merchant.set_last_new_sub_plan(Some(plan.clone()));
                        
                        // Determine billing cycle
                        re = Regex::new(YEARLY_PATTERN).unwrap();
                        if re.is_match(event.details().as_str()) {
                            merchant.set_last_new_sub_billing_cycle(Some(BillingCycle::Yearly));
                        } else {
                            merchant.set_last_new_sub_billing_cycle(Some(BillingCycle::Monthly));
                        }
                        
                        total_stats.sub_stats_details_mut().all_new_sub_mut().increase(
                            plan, 
                            merchant.last_new_sub_billing_cycle().as_ref().unwrap(), 
                            1)
                            .unwrap();
                        
                        // Determine if the event stands for an active subscription
                        if merchant.subscription_status() == SUBSCRIPTION_STATUS_ACTIVE {
                            total_stats.sub_stats_details_mut().new_sub_mut().increase(
                                plan, 
                                merchant.last_new_sub_billing_cycle().as_ref().unwrap(), 
                                1).
                                unwrap();
                        }

                        break;
                    }
                
                }

                break;
            }
        }

        // Determine canceled subscription details
        for event in merchant.clone().subscription_events().iter() { // Use normal order to get the earliest canceled event
            if SUBSCRIPTION_CANCELED_STRINGS.contains(&event.event().as_str()) {
                // Determine plan
                for plan in pricing_defs.subscriptions() {
                    let mut re = Regex::new(plan.regex_pattern().as_str()).unwrap();
                    if re.is_match(event.details().as_str()) {
                        merchant.set_first_canceled_sub_plan(Some(plan.clone()));
                        
                        // Determine billing cycle
                        re = Regex::new(YEARLY_PATTERN).unwrap();
                        if re.is_match(event.details().as_str()) {
                            merchant.set_first_canceled_sub_billing_cycle(Some(BillingCycle::Yearly));
                        } else {
                            merchant.set_first_canceled_sub_billing_cycle(Some(BillingCycle::Monthly));
                        }
                        
                        total_stats.sub_stats_details_mut().all_canceled_sub_mut().increase(
                            plan, 
                            merchant.first_canceled_sub_billing_cycle().as_ref().unwrap(), 
                            1)
                            .unwrap();
                        
                        // Determine if the event stands for a canceled subscription
                        if merchant.subscription_status() == SUBSCRIPTION_STATUS_CANCELED {
                            total_stats.sub_stats_details_mut().canceled_sub_mut().increase(
                                plan, 
                                merchant.first_canceled_sub_billing_cycle().as_ref().unwrap(), 
                                1).
                                unwrap();
                        }

                        break;
                    }
                
                }

                break;
            }
        }

        // Update final total data
        total_stats.set_churn_rate(
            if *total_stats.installed_count() > 0 {
                (*total_stats.uninstalled_count() as f64 - *total_stats.old_uninstalled_count() as f64)
                / *total_stats.installed_count() as f64 
                * 100.0
            } else {
                0.0
            }
        );

        total_stats.set_sub_growth(
            *total_stats.new_sub_count() as i32 - *total_stats.canceled_sub_count() as i32
        );

        total_stats.set_paid_growth(
            total_stats.sub_growth() + *total_stats.one_time_count() as i32
        );

        
        // Calculate subscription growth details
        let mut calculated_result: HashMap<String, i32> = HashMap::new();
        
            // Yearly
        for (plan, new_count) in total_stats.clone().sub_stats_details().new_sub().yearly_counts() {
            let canceled_count = total_stats.sub_stats_details().canceled_sub().yearly_counts().get(plan).unwrap_or(&0);
            calculated_result.insert(plan.to_string(), new_count - canceled_count);
        }
        total_stats.sub_stats_details_mut().sub_growth_mut().set_yearly_counts(calculated_result.clone());
        
            // Monthly
        calculated_result.clear();
        for (plan, new_count) in total_stats.clone().sub_stats_details().new_sub().monthly_counts() {
            let canceled_count = total_stats.sub_stats_details().canceled_sub().monthly_counts().get(plan).unwrap_or(&0);
            calculated_result.insert(plan.to_string(), new_count - canceled_count);
        }
        total_stats.sub_stats_details_mut().sub_growth_mut().set_monthly_counts(calculated_result.clone());
    
    }

    Ok(())
}

pub fn analyze_events_list(
    event_list: &Vec<AppEvent>, 
    pricing_defs: &PricingDefs, 
    excluding_defs: &ExcludingDef) 
    -> anyhow::Result<(TotalStats, MerchantDataList)> {
    
    let (mut total_stats, mut merchant_data) = build_merchant_data_and_count_basic_stats(event_list, pricing_defs, excluding_defs);

    process_merchant_data_and_count_final_stats(&mut total_stats, &mut merchant_data, pricing_defs);

    Ok((total_stats, merchant_data))
}

pub fn process_from_files_to_files(
    event_history_file: &PathBuf,
    pricing_defs_file: Option<&PathBuf>,
    excluding_defs_file: Option<&PathBuf>,
    out_file_total_stats: &PathBuf,
    out_file_merchant_data: Option<&PathBuf>,
    out_file_app_event: Option<&PathBuf>)
    -> anyhow::Result<(String)> {
    
    let event_list: Vec<AppEvent> = read_events_from_csv(event_history_file)?;

    let pricing_defs: PricingDefs;
    let excluding_defs: ExcludingDef;
    
    if let Some(f) = pricing_defs_file {
        pricing_defs = read_pricing_def_from_json(f)?;
    } else {
        pricing_defs = read_pricing_def_from_json_str(SBM_PRICING_DEF_JSON_STRING)?;
    }

    if let Some(f) = excluding_defs_file {
        excluding_defs = read_excluding_def_from_json(f)?;
    } else {
        excluding_defs = read_excluding_def_from_json_str(MS_EXCLUDING_DEF_JSON_STRING)?;
    }
    
    let (total_stats, merchant_data) = analyze_events_list(&event_list, &pricing_defs, &excluding_defs)?;

    let mut message_success: String;

    match write_total_stats_to_json(out_file_total_stats, &total_stats) {
        Ok(()) => message_success = data::TOTAL_STATS.to_string() 
                                + success::success::SPECIFIC_DATA_WRITTEN
                                + out_file_total_stats.display().to_string().as_str(),
        Err(e) => return Err(e)
    }

    if let Some(f) = out_file_merchant_data {
        match write_merchant_data_to_json(f, &merchant_data) {
            Ok(()) => message_success = message_success 
                                        + data::MERCHANT_DATA
                                        + success::success::SPECIFIC_DATA_WRITTEN
                                        + f.display().to_string().as_str(),
            Err(e) => return Err(e)
        }
    }

    if let Some(f) =  out_file_app_event {
        match write_app_event_list_to_json(f, &event_list) {
            Ok(()) => message_success = message_success 
                                        + data::APP_EVENTS
                                        + success::success::SPECIFIC_DATA_WRITTEN
                                        + f.display().to_string().as_str(),
            Err(e) => return Err(e)
        }
    }

    Ok(message_success)
}