use anyhow::anyhow;
use indexmap::IndexMap;
use regex::Regex;
use std::path::PathBuf;

use crate::data_io::*;
use crate::definitions::common::*;
use crate::definitions::strings::*;
use crate::models::data_model::*;
use crate::models::ui_model::*;

/// Build base data from app event list
/// Base data include:
/// - All installing related data (install, uninstall, store-closed, churn rate,...)
/// - One-time data
fn build_base_data(
    app_event_list: &Vec<AppEvent>,
    pricing_defs: &PricingDefs,
    excluding_def: &ExcludingDef,
    case_sensitive_regex: bool,
) -> (TotalStats, MerchantList) {
    let mut merchant_list: MerchantList = MerchantList::new();
    let mut total_stats: TotalStats = TotalStats::new(pricing_defs);

    total_stats.set_start_time(app_event_list.first().unwrap().time().clone());
    total_stats.set_end_time(app_event_list.last().unwrap().time().clone());
    merchant_list.set_start_time(app_event_list.first().unwrap().time().clone());
    merchant_list.set_end_time(app_event_list.last().unwrap().time().clone());

    total_stats.build_pretty_time_str();

    for event in app_event_list {
        // Excluding check
        let mut re = if case_sensitive_regex {
            Regex::new(excluding_def.excluding_pattern()).unwrap()
        } else {
            Regex::new(excluding_def.excluding_pattern().to_lowercase().as_str()).unwrap()
        };

        if re.is_match(&event.excluding_check_data().as_str())
            || (!case_sensitive_regex
                && re.is_match(event.excluding_check_data().to_lowercase().as_str()))
        {
            continue;
        }

        // If merchant in this event existed in merchant list, clone to edit; else create new
        let mut current_merchant = if merchant_list
            .merchants()
            .contains_key(event.shop_domain().as_str())
        {
            merchant_list
                .merchants()
                .get(event.shop_domain().as_str())
                .unwrap()
                .clone()
        } else {
            Merchant::new(&event.shop_domain(), pricing_defs.one_times())
        };

        // Count Install, Uninstall, Store closed
        re = Regex::new(INSTALLED_STRING).unwrap();
        if re.is_match(event.event().as_str()) {
            total_stats.increase_installed_count(1);
            current_merchant.increase_installed_count(1);
            current_merchant.push_installing_event(event);
            merchant_list.update_merchant(current_merchant);
            continue;
        }

        re = Regex::new(UNINSTALLED_STRING).unwrap();
        if re.is_match(event.event().as_str()) {
            total_stats.increase_uninstalled_count(1);
            current_merchant.increase_uninstalled_count(1);
            current_merchant.push_installing_event(event);
            merchant_list.update_merchant(current_merchant);
            continue;
        }

        re = Regex::new(STORE_CLOSED_STRING).unwrap();
        if re.is_match(event.event().as_str()) {
            total_stats.increase_store_closed_count(1);
            current_merchant.increase_store_closed_count(1);
            current_merchant.push_installing_event(event);
            merchant_list.update_merchant(current_merchant);
            continue;
        }

        re = Regex::new(STORE_REOPENED_STRING).unwrap();
        if re.is_match(event.event().as_str()) {
            total_stats.increase_store_reopened_count(1);
            current_merchant.increase_store_reopened_count(1);
            current_merchant.push_installing_event(event);
            merchant_list.update_merchant(current_merchant);
            continue;
        }

        //  Count One-Time
        if ONE_TIME_ACTIVATED_STRINGS.contains(&event.event().as_str()) {
            total_stats.increase_one_time_count(1);
            current_merchant.increase_one_time_count(1);
            current_merchant.push_one_time_event(event);

            for pack in pricing_defs.one_times() {
                re = if case_sensitive_regex {
                    Regex::new(pack.regex_pattern().as_str()).unwrap()
                } else {
                    Regex::new(pack.regex_pattern().to_lowercase().as_str()).unwrap()
                };

                if re.is_match(event.details().as_str())
                    || (!case_sensitive_regex
                        && re.is_match(event.details().to_lowercase().as_str()))
                {
                    total_stats.increase_one_time_pack_count(pack, 1).unwrap();
                    current_merchant
                        .increase_one_time_pack_count(pack, 1)
                        .unwrap();
                    break;
                }
            }

            // Push back to merchant data.
            // If current merchant existed in merchant list, update function will update new detailed data to existing element, else push to list as new merchant
            merchant_list.update_merchant(current_merchant);
            continue;
        }

        //  Count Subscriptions
        if SUBSCRIPTION_ACTIVATED_STRINGS.contains(&event.event().as_str()) {
            current_merchant.increase_subscription_activated_count(1);
            current_merchant.push_subscription_event(event);
            merchant_list.update_merchant(current_merchant);
            continue;
        }
        if SUBSCRIPTION_CANCELED_STRINGS.contains(&event.event().as_str()) {
            current_merchant.increase_subscription_canceled_count(1);
            current_merchant.push_subscription_event(event);
            merchant_list.update_merchant(current_merchant);
            continue;
        }
    }

    // Calculate more stats without merchant data analyzing
    total_stats.set_merchant_growth(
        *total_stats.installed_count() as i32 + *total_stats.store_reopened_count() as i32
            - *total_stats.uninstalled_count() as i32
            - *total_stats.store_closed_count() as i32,
    );

    total_stats.set_total_churn_rate(if *total_stats.installed_count() > 0 {
        (*total_stats.uninstalled_count() as f64 / *total_stats.installed_count() as f64) * 100.0
    } else {
        0.0
    });

    (total_stats, merchant_list)
}

/// Analyze subscription-related data from base data
/// Update the base data in place so that it becomes the final data.
fn analyze_details(
    total_stats: &mut TotalStats,
    merchant_list: &mut MerchantList,
    pricing_defs: &PricingDefs,
    case_sensitive_regex: bool,
) {
    //  Process merchant data
    for merchant in merchant_list.merchants_mut().values_mut() {
        //  Updated installed status
        match *merchant.installed_count() as i32 + *merchant.store_reopened_count() as i32
            - *merchant.uninstalled_count() as i32
            - *merchant.store_closed_count() as i32
        {
            delta if delta > 0 => {
                merchant.set_installed_status(INSTALLED_STRING.to_string());
            }
            delta if delta < 0 => {
                merchant.set_installed_status(UNINSTALLED_STRING.to_string());
                if merchant.installing_events().len() > 0
                    && merchant.installing_events().first().unwrap().event() == UNINSTALLED_STRING
                {
                    merchant.set_installed_status(UNINSTALLED_OLD_STRING.to_string());
                    total_stats.increase_old_uninstalled_count(1);
                }
            }
            _ => {
                merchant.set_installed_status(NONE.to_string());
            }
        }

        //  Determine final subscription status
        match *merchant.subscription_activated_count() as i32
            - *merchant.subscription_canceled_count() as i32
        {
            delta if delta > 0 => {
                merchant.set_subscription_status(SUBSCRIPTION_STATUS_ACTIVE.to_string());
                total_stats.increase_new_sub_count(1);
            }
            delta if delta < 0 => {
                merchant.set_subscription_status(SUBSCRIPTION_STATUS_CANCELED.to_string());
                total_stats.increase_canceled_sub_count(1);
            }
            _ => {
                merchant.set_subscription_status(NONE.to_string());
            }
        }

        //  Determine new subscription details
        for event in merchant.clone().subscription_events().iter().rev() {
            //  Use reverse order to get the latest activated event
            if SUBSCRIPTION_ACTIVATED_STRINGS.contains(&event.event().as_str()) {
                //  Determine plan
                for plan in pricing_defs.subscriptions() {
                    let mut re = if case_sensitive_regex {
                        Regex::new(plan.regex_pattern().as_str()).unwrap()
                    } else {
                        Regex::new(plan.regex_pattern().to_lowercase().as_str()).unwrap()
                    };

                    if re.is_match(event.details().as_str())
                        || (!case_sensitive_regex
                            && re.is_match(&event.details().to_lowercase().as_str()))
                    {
                        merchant.set_last_new_sub_plan(Some(plan.clone()));

                        //  Determine billing cycle
                        re = if case_sensitive_regex {
                            Regex::new(YEARLY_PATTERN).unwrap()
                        } else {
                            Regex::new(YEARLY_PATTERN.to_lowercase().as_str()).unwrap()
                        };

                        if re.is_match(event.details().as_str())
                            || (!case_sensitive_regex
                                && re.is_match(&event.details().to_lowercase().as_str()))
                        {
                            merchant.set_last_new_sub_billing_cycle(Some(BillingCycle::Yearly));
                        } else {
                            merchant.set_last_new_sub_billing_cycle(Some(BillingCycle::Monthly));
                        }

                        total_stats
                            .sub_stats_details_mut()
                            .all_new_sub_mut()
                            .increase(
                                plan,
                                merchant.last_new_sub_billing_cycle().as_ref().unwrap(),
                                1,
                            )
                            .unwrap(); // Always Ok, because pricing definitions are not modified anywhere in whole program

                        //  Determine if the event stands for an active subscription
                        if merchant.subscription_status() == SUBSCRIPTION_STATUS_ACTIVE {
                            total_stats
                                .sub_stats_details_mut()
                                .new_sub_mut()
                                .increase(
                                    plan,
                                    merchant.last_new_sub_billing_cycle().as_ref().unwrap(),
                                    1,
                                )
                                .unwrap(); // Always Ok, because pricing definitions are not modified anywhere in whole program
                        }

                        break;
                    }
                }

                break;
            }
        }

        //  Determine canceled subscription details
        for event in merchant.clone().subscription_events().iter() {
            //  Use normal order to get the earliest canceled event
            if SUBSCRIPTION_CANCELED_STRINGS.contains(&event.event().as_str()) {
                //  Determine plan
                for plan in pricing_defs.subscriptions() {
                    let mut re = Regex::new(plan.regex_pattern().as_str()).unwrap();
                    if re.is_match(event.details().as_str()) {
                        merchant.set_first_canceled_sub_plan(Some(plan.clone()));

                        //  Determine billing cycle
                        re = Regex::new(YEARLY_PATTERN).unwrap();
                        if re.is_match(event.details().as_str()) {
                            merchant
                                .set_first_canceled_sub_billing_cycle(Some(BillingCycle::Yearly));
                        } else {
                            merchant
                                .set_first_canceled_sub_billing_cycle(Some(BillingCycle::Monthly));
                        }

                        total_stats
                            .sub_stats_details_mut()
                            .all_canceled_sub_mut()
                            .increase(
                                plan,
                                merchant
                                    .first_canceled_sub_billing_cycle()
                                    .as_ref()
                                    .unwrap(),
                                1,
                            )
                            .unwrap(); // Always Ok, because pricing definitions are not modified anywhere in whole program

                        //  Determine if the event stands for a canceled subscription
                        if merchant.subscription_status() == SUBSCRIPTION_STATUS_CANCELED {
                            total_stats
                                .sub_stats_details_mut()
                                .canceled_sub_mut()
                                .increase(
                                    plan,
                                    merchant
                                        .first_canceled_sub_billing_cycle()
                                        .as_ref()
                                        .unwrap(),
                                    1,
                                )
                                .unwrap(); // Always Ok, because pricing definitions are not modified anywhere in whole program
                        }

                        break;
                    }
                }

                break;
            }
        }

        //  Update final total data
        total_stats.set_churn_rate(if *total_stats.installed_count() > 0 {
            (*total_stats.uninstalled_count() as f64 - *total_stats.old_uninstalled_count() as f64)
                / *total_stats.installed_count() as f64
                * 100.0
        } else {
            0.0
        });

        total_stats.set_sub_growth(
            *total_stats.new_sub_count() as i32 - *total_stats.canceled_sub_count() as i32,
        );

        total_stats
            .set_paid_growth(total_stats.sub_growth() + *total_stats.one_time_count() as i32);

        //  Calculate subscription growth details
        let mut calculated_result: IndexMap<String, i32> = IndexMap::new();

        //  Yearly
        for (plan, new_count) in total_stats
            .clone()
            .sub_stats_details()
            .new_sub()
            .yearly_counts()
        {
            let canceled_count = total_stats
                .sub_stats_details()
                .canceled_sub()
                .yearly_counts()
                .get(plan)
                .unwrap_or(&0);
            calculated_result.insert(plan.to_string(), new_count - canceled_count);
        }
        total_stats
            .sub_stats_details_mut()
            .sub_growth_mut()
            .set_yearly_counts(calculated_result.clone());

        //  Monthly
        calculated_result.clear();
        for (plan, new_count) in total_stats
            .clone()
            .sub_stats_details()
            .new_sub()
            .monthly_counts()
        {
            let canceled_count = total_stats
                .sub_stats_details()
                .canceled_sub()
                .monthly_counts()
                .get(plan)
                .unwrap_or(&0);
            calculated_result.insert(plan.to_string(), new_count - canceled_count);
        }
        total_stats
            .sub_stats_details_mut()
            .sub_growth_mut()
            .set_monthly_counts(calculated_result.clone());
    }
}

/// Analyze event list and return final data
pub fn analyze_events_list(
    event_list: &Vec<AppEvent>,
    pricing_defs: &PricingDefs,
    excluding_defs: &ExcludingDef,
    case_sensitive_regex: bool,
) -> anyhow::Result<(TotalStats, MerchantList)> {
    let (mut total_stats, mut merchant_data) = build_base_data(
        event_list,
        pricing_defs,
        excluding_defs,
        case_sensitive_regex,
    );

    analyze_details(
        &mut total_stats,
        &mut merchant_data,
        pricing_defs,
        case_sensitive_regex,
    );

    Ok((total_stats, merchant_data))
}

fn analyze_file(
    event_history_file: &PathBuf,
    pricing_defs: &PricingDefs,
    excluding_defs: &ExcludingDef,
    case_sensitive_regex: bool,
    out_folder: &PathBuf,
    out_file_total_stats_pref: &str,
    out_file_merchant_data_pref: &Option<String>,
    out_file_app_events_pref: &Option<String>,
) -> anyhow::Result<String> {
    let event_list: Vec<AppEvent> =
        read_events_from_csv(event_history_file, &excluding_defs.excluding_field())?;

    let (total_stats, merchant_data) = analyze_events_list(
        &event_list,
        pricing_defs,
        excluding_defs,
        case_sensitive_regex,
    )?;

    let mut message_success: String;

    let out_file_total_stats: PathBuf = out_folder.join(format!(
        "{}_{}_{}.json",
        out_file_total_stats_pref,
        total_stats.start_time_str(),
        total_stats.end_time_str()
    ));

    match write_total_stats_to_json(&out_file_total_stats, &total_stats) {
        Ok(()) => {
            message_success = data::TOTAL_STATS.to_string()
                + message::success::SPECIFIC_DATA_WRITTEN_FILE
                + out_file_total_stats.display().to_string().as_str()
        }
        Err(e) => return Err(e),
    }

    if let Some(pref) = out_file_merchant_data_pref {
        let out_file_merchant_data: PathBuf = out_folder.join(format!(
            "{}_{}_{}.json",
            pref,
            total_stats.start_time_str(),
            total_stats.end_time_str()
        ));
        match write_merchant_data_to_json(&out_file_merchant_data, &merchant_data) {
            Ok(()) => {
                message_success = message_success
                    + data::MERCHANT_DATA
                    + message::success::SPECIFIC_DATA_WRITTEN_FILE
                    + out_file_merchant_data.display().to_string().as_str()
            }
            Err(e) => return Err(e),
        }
    }

    if let Some(pref) = out_file_app_events_pref {
        let out_file_app_events: PathBuf = out_folder.join(format!(
            "{}_{}_{}.json",
            pref,
            total_stats.start_time_str(),
            total_stats.end_time_str()
        ));
        match write_app_event_list_to_json(&out_file_app_events, &event_list) {
            Ok(()) => {
                message_success = message_success
                    + data::APP_EVENTS
                    + message::success::SPECIFIC_DATA_WRITTEN_FILE
                    + out_file_app_events.display().to_string().as_str()
            }
            Err(e) => return Err(e),
        }
    }

    Ok(message_success)
}

pub fn analyze_from_gui(
    event_history_file_list: &Option<Vec<PathBuf>>,
    selected_pricing_defs_option: &UiOption,
    selected_excluding_defs_option: &UiOption,
    pricing_defs_file: &Option<PathBuf>,
    excluding_defs_file: &Option<PathBuf>,
    debug_mode: bool,
    case_sensitive_regex: bool,
) -> anyhow::Result<String> {
    let pricing_defs: PricingDefs;
    let excluding_defs: ExcludingDef;

    pricing_defs = match selected_pricing_defs_option {
        d if d.value() == ui::OPTION_CUSTOM.value() => {
            if let Some(f) = pricing_defs_file {
                read_pricing_def_from_json(&f)?
            } else {
                return Err(anyhow!(
                    "{} {} {}!",
                    data::KIND_CUSTOM.to_string(),
                    data::PRICING_DEFS,
                    message::error::FILE_NOT_CHOSEN
                ));
            }
        }
        _ => read_pricing_def_from_json_str(
            selected_pricing_defs_option
                .connected_data()
                .as_ref()
                .unwrap(),
        )?,
    };

    excluding_defs = match selected_excluding_defs_option {
        d if d.value() == ui::OPTION_CUSTOM.value() => {
            if let Some(f) = excluding_defs_file {
                read_excluding_def_from_json(&f)?
            } else {
                return Err(anyhow!(
                    "{} {} {}!",
                    data::KIND_CUSTOM.to_string(),
                    data::EXCLUDING_DEFS,
                    message::error::FILE_NOT_CHOSEN
                ));
            }
        }
        _ => read_excluding_def_from_json_str(
            selected_excluding_defs_option
                .connected_data()
                .as_ref()
                .unwrap(),
        )?,
    };

    if let Some(f_list) = event_history_file_list {
        let out_folder: PathBuf = std::env::current_dir()?.join(data::OUT_FOLDER_NAME);
        let mut final_error_message: String = String::from("");
        let mut final_success_message: String = format!(
            "{} {}\n",
            data::TOTAL_STATS.to_string(),
            message::success::SPECIFIC_DATA_WRITTEN_FILE
        );

        let out_file_total_stats_pref: String = data::TOTAL_STATS
            .to_string()
            .replace(" ", "_")
            .to_lowercase();

        let mut out_file_merchant_data_pref: Option<String> = None;
        let mut out_file_app_events_pref: Option<String> = None;

        if debug_mode {
            final_success_message += format!(
                "{} {}\n{} {}",
                data::MERCHANT_DATA,
                message::success::SPECIFIC_DATA_WRITTEN_FILE,
                data::APP_EVENTS,
                message::success::SPECIFIC_DATA_WRITTEN_FILE
            )
            .as_str();

            out_file_merchant_data_pref = Some(
                data::MERCHANT_DATA
                    .to_string()
                    .replace(" ", "_")
                    .to_lowercase(),
            );

            out_file_app_events_pref = Some(
                data::APP_EVENTS
                    .to_string()
                    .replace(" ", "_")
                    .to_lowercase(),
            );
        }

        for f in f_list {
            if let Err(e) = analyze_file(
                f,
                &pricing_defs,
                &excluding_defs,
                case_sensitive_regex,
                &out_folder,
                &out_file_total_stats_pref,
                &out_file_merchant_data_pref,
                &out_file_app_events_pref,
            ) {
                final_error_message += e.to_string().as_str();
            }
        }

        if final_error_message != "" {
            return Err(anyhow!(final_error_message));
        }

        return Ok(final_success_message);
    } else {
        return Err(anyhow!(
            "{} {}!",
            data::APP_EVENTS.to_string(),
            message::error::FILE_NOT_CHOSEN
        ));
    }
}
