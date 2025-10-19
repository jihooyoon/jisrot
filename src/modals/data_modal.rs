use crate::definitions::common::*;
use anyhow::{Result, anyhow};
use chrono::{NaiveDate, NaiveDateTime};
use getset::{Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingCycle {
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, Getters, Setters, Serialize, Deserialize)]
#[getset(get = "pub", set = "pub")]
pub struct MerchantData {
    shop_domain: String,
    checked: bool,
    installed_count: u32,
    uninstalled_count: u32,
    store_closed_count: u32,
    store_reopened_count: u32,

    #[getset(get = "pub", set = "")]
    installing_events: Vec<AppEvent>,

    subscription_activated_count: u32,
    subscription_canceled_count: u32,

    #[getset(get = "pub", set = "")]
    subscription_events: Vec<AppEvent>,

    one_time_count: u32,

    #[getset(skip)]
    one_time_details: HashMap<String, u32>,

    #[getset(skip)]
    one_time_events: Vec<AppEvent>,

    installed_status: String,
    subscription_status: String,
    last_new_sub_plan: Option<PricingUnit>,
    last_new_sub_billing_cycle: Option<BillingCycle>,
    first_canceled_sub_plan: Option<PricingUnit>,
    first_canceled_sub_billing_cycle: Option<BillingCycle>,
}

impl MerchantData {
    pub fn new(shop_domain: &String, one_time_packs: &Vec<PricingUnit>) -> Self {
        let mut one_time_details: HashMap<String, u32> = HashMap::new();

        for pack in one_time_packs.iter() {
            one_time_details.insert(pack.code.clone(), 0);
        }

        Self {
            shop_domain: shop_domain.clone(),
            checked: false,
            installed_count: 0,
            uninstalled_count: 0,
            store_closed_count: 0,
            store_reopened_count: 0,
            installing_events: Vec::new(),
            subscription_activated_count: 0,
            subscription_canceled_count: 0,
            subscription_events: Vec::new(),
            one_time_count: 0,
            one_time_details: one_time_details,
            one_time_events: Vec::new(),
            installed_status: NONE.to_string(),
            subscription_status: NONE.to_string(),
            last_new_sub_plan: None,
            last_new_sub_billing_cycle: None,
            first_canceled_sub_plan: None,
            first_canceled_sub_billing_cycle: None,
        }
    }

    pub fn increase_one_time_count(&mut self, count: u32) {
        self.one_time_count += count;
    }

    pub fn increase_one_time_pack_count(
        &mut self,
        pack: &PricingUnit,
        count: u32,
    ) -> anyhow::Result<()> {
        if let Some(entry) = self.one_time_details.get_mut(&pack.code) {
            *entry += count;
            Ok(())
        } else {
            Err(anyhow!(
                "[Merchant Data] One-time pack code {} not found in initialized one-time count stats",
                pack.code
            ))
        }
    }

    pub fn increase_installed_count(&mut self, count: u32) {
        self.installed_count += count;
    }

    pub fn increase_uninstalled_count(&mut self, count: u32) {
        self.uninstalled_count += count;
    }

    pub fn increase_store_closed_count(&mut self, count: u32) {
        self.store_closed_count += count;
    }

    pub fn increase_store_reopened_count(&mut self, count: u32) {
        self.store_reopened_count += count;
    }

    pub fn increase_subscription_canceled_count(&mut self, count: u32) {
        self.subscription_canceled_count += count;
    }

    pub fn increase_subscription_activated_count(&mut self, count: u32) {
        self.subscription_activated_count += count;
    }

    pub fn push_subscription_event(&mut self, event: &AppEvent) {
        self.subscription_events.push(event.clone());
    }

    pub fn push_one_time_event(&mut self, event: &AppEvent) {
        self.one_time_events.push(event.clone());
    }

    pub fn push_installing_event(&mut self, event: &AppEvent) {
        self.installing_events.push(event.clone());
    }
}

#[derive(Debug, Clone, Getters, MutGetters, Setters, Serialize, Deserialize)]
#[getset(get = "pub", set = "pub")]
pub struct MerchantDataList {
    start_time: Option<NaiveDateTime>,
    end_time: Option<NaiveDateTime>,

    #[getset(get = "pub", get_mut = "pub", set = "")]
    merchants: HashMap<String, MerchantData>,
}

impl MerchantDataList {
    pub fn new() -> Self {
        Self {
            start_time: None,
            end_time: None,
            merchants: HashMap::new(),
        }
    }

    pub fn update_merchant(&mut self, merchant: MerchantData) {
        self.merchants
            .insert(merchant.shop_domain().clone(), merchant);
    }
}

#[derive(Debug, Clone, Getters, MutGetters, Setters, Serialize, Deserialize)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct TotalStats {
    start_time: Option<NaiveDateTime>,
    end_time: Option<NaiveDateTime>,
    start_time_str: String,
    end_time_str: String,

    installed_count: u32,
    uninstalled_count: u32,
    old_uninstalled_count: u32,
    total_churn_rate: f64,
    churn_rate: f64,
    merchant_growth: i32,

    store_closed_count: u32,
    store_reopened_count: u32,

    one_time_count: u32,

    #[getset(skip)]
    one_time_details: HashMap<String, u32>,

    new_sub_count: u32,
    canceled_sub_count: u32,
    sub_growth: i32,

    sub_stats_details: DetailedSubscriptionStats,

    paid_growth: i32,
}

impl TotalStats {
    pub fn new(pricing_defs: &PricingDefs) -> Self {
        let mut one_time_details: HashMap<String, u32> = HashMap::new();

        for pack in pricing_defs.one_times.iter() {
            one_time_details.insert(pack.code.clone(), 0);
        }

        Self {
            start_time: None,
            end_time: None,
            start_time_str: NONE.to_string(),
            end_time_str: NONE.to_string(),
            installed_count: 0,
            uninstalled_count: 0,
            old_uninstalled_count: 0,
            total_churn_rate: 0.0,
            churn_rate: 0.0,
            merchant_growth: 0,
            store_closed_count: 0,
            store_reopened_count: 0,
            one_time_count: 0,
            one_time_details: one_time_details,
            new_sub_count: 0,
            canceled_sub_count: 0,
            sub_growth: 0,
            sub_stats_details: DetailedSubscriptionStats::new(&pricing_defs.subscriptions),
            paid_growth: 0,
        }
    }

    pub fn build_pretty_time_str(&mut self) {
        if let Some(t) = self.start_time {
            self.start_time_str = t.format("%b%d").to_string();
        }
        if let Some(t) = self.end_time {
            self.end_time_str = t.format("%b%d").to_string();
        }
    }

    pub fn increase_one_time_count(&mut self, count: u32) {
        self.one_time_count += count;
    }

    pub fn increase_one_time_pack_count(
        &mut self,
        pack: &PricingUnit,
        count: u32,
    ) -> anyhow::Result<()> {
        if let Some(entry) = self.one_time_details.get_mut(&pack.code) {
            *entry += count;
            Ok(())
        } else {
            Err(anyhow!(
                "[TotalStats] One-time pack code {} not found in initialized one-time count stats",
                pack.code
            ))
        }
    }

    pub fn increase_installed_count(&mut self, count: u32) {
        self.installed_count += count;
    }

    pub fn increase_uninstalled_count(&mut self, count: u32) {
        self.uninstalled_count += count;
    }

    pub fn increase_old_uninstalled_count(&mut self, count: u32) {
        self.old_uninstalled_count += count;
    }

    pub fn increase_store_closed_count(&mut self, count: u32) {
        self.store_closed_count += count;
    }

    pub fn increase_store_reopened_count(&mut self, count: u32) {
        self.store_reopened_count += count;
    }

    pub fn increase_new_sub_count(&mut self, count: u32) {
        self.new_sub_count += count;
    }

    pub fn increase_canceled_sub_count(&mut self, count: u32) {
        self.canceled_sub_count += count;
    }
}

#[derive(Debug, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct ExcludingDef {
    excluding_field: String,
    excluding_pattern: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct PricingUnit {
    code: String,
    name: String,
    regex_pattern: String,
    price: f64,
    currency: String,
}

#[derive(Debug, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct PricingDefs {
    subscriptions: Vec<PricingUnit>,
    one_times: Vec<PricingUnit>,
}

#[derive(Debug, Setters, Getters, MutGetters, Serialize, Deserialize, Clone)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct SubscriptionStatsCounter {
    monthly_counts: HashMap<String, i32>,
    yearly_counts: HashMap<String, i32>,
}

impl SubscriptionStatsCounter {
    pub fn new(subscription_plan_list: &Vec<PricingUnit>) -> Self {
        let mut monthly_counts: HashMap<String, i32> = HashMap::new();
        let mut yearly_counts: HashMap<String, i32> = HashMap::new();

        for subscription_plan in subscription_plan_list.iter() {
            monthly_counts.insert(subscription_plan.code.clone(), 0);
            yearly_counts.insert(subscription_plan.code.clone(), 0);
        }

        Self {
            monthly_counts: monthly_counts,
            yearly_counts: yearly_counts,
        }
    }

    pub fn increase(
        &mut self,
        subscription_plan: &PricingUnit,
        billing_cycle: &BillingCycle,
        count: i32,
    ) -> anyhow::Result<()> {
        match billing_cycle {
            BillingCycle::Monthly => {
                if let Some(entry) = self.monthly_counts.get_mut(&subscription_plan.code) {
                    *entry += count;
                    Ok(())
                } else {
                    Err(anyhow!(
                        "[SubscriptionStatsCounter] Subscription plan code {} not found in initialized monthly count stats",
                        subscription_plan.code
                    ))
                }
            }

            BillingCycle::Yearly => {
                if let Some(entry) = self.yearly_counts.get_mut(&subscription_plan.code) {
                    *entry += count;
                    Ok(())
                } else {
                    Err(anyhow!(
                        "[SubscriptionStatsCounter] Subscription plan code {} not found in initialized yearly count stats",
                        subscription_plan.code
                    ))
                }
            }
        }
    }
}

#[derive(Debug, Getters, MutGetters, Setters, Serialize, Deserialize, Clone)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct DetailedSubscriptionStats {
    new_sub: SubscriptionStatsCounter,
    canceled_sub: SubscriptionStatsCounter,
    sub_growth: SubscriptionStatsCounter,
    all_new_sub: SubscriptionStatsCounter,
    all_canceled_sub: SubscriptionStatsCounter,
}

impl DetailedSubscriptionStats {
    pub fn new(subscription_plan_list: &Vec<PricingUnit>) -> Self {
        Self {
            new_sub: SubscriptionStatsCounter::new(subscription_plan_list),
            canceled_sub: SubscriptionStatsCounter::new(subscription_plan_list),
            sub_growth: SubscriptionStatsCounter::new(subscription_plan_list),
            all_new_sub: SubscriptionStatsCounter::new(subscription_plan_list),
            all_canceled_sub: SubscriptionStatsCounter::new(subscription_plan_list),
        }
    }
}

#[derive(Debug, Clone, Getters, Setters, Serialize, Deserialize)]
#[getset(get = "pub", set = "pub")]
pub struct AppEvent {
    time: Option<NaiveDateTime>,
    event: String,
    details: String,
    billing_on: Option<NaiveDateTime>,
    shop_name: String,
    shop_country: String,
    shop_email: String,
    shop_domain: String,
    key: String,
}

impl AppEvent {
    pub fn new() -> Self {
        AppEvent {
            time: None,
            event: String::default(),
            details: String::default(),
            billing_on: None,
            shop_name: String::default(),
            shop_country: String::default(),
            shop_email: String::default(),
            shop_domain: String::default(),
            key: String::default(),
        }
    }

    fn parse_time(
        data_hash: &HashMap<String, String>,
        data_field: &str,
        pattern: &str,
    ) -> Result<NaiveDateTime, String> {
        if let Some(time_string) = data_hash.get(data_field) {
            match NaiveDateTime::parse_from_str(time_string.as_str(), pattern) {
                Ok(date_time) => Ok(date_time),

                Err(e) => {
                    eprintln!("Date Time Parse error: {:?}\n Trying parse Date only...", e);
                    match NaiveDate::parse_from_str(time_string.as_str(), pattern) {
                        Ok(date) => {
                            if let Some(date_time_fk) = date.and_hms_opt(0, 0, 0) {
                                Ok(date_time_fk)
                            } else {
                                Err(format!(
                                    "Invalid date format for field {}: {}",
                                    data_field, time_string
                                ))
                            }
                        }
                        Err(e) => {
                            eprintln!("Date Parse error: {:?}", e);
                            Err(format!(
                                "Invalid date format for field {}: {}",
                                data_field, time_string
                            ))
                        }
                    }
                }
            }
        } else {
            return Err(format!("Missing required field: {}", data_field));
        }
    }

    pub fn from_hashmap(hashmap: &HashMap<String, String>) -> Result<Self, String> {
        let mut time = None;
        let mut billing_on = None;

        match Self::parse_time(hashmap, TIME_FIELD, EVENT_TIME_PATTERN) {
            Ok(date_time) => time = Some(date_time),
            Err(e) => println!("Warning: {}", e),
        }

        match Self::parse_time(hashmap, BILLING_ON_FIELD, BILLING_ON_PATTERN) {
            Ok(date_time) => billing_on = Some(date_time),
            Err(e) => println!("Warning: {}", e),
        }

        Ok(AppEvent {
            time,
            event: hashmap.get(EVENT_FIELD).cloned().unwrap_or_default(),
            details: hashmap.get(DETAILS_FIELD).cloned().unwrap_or_default(),
            billing_on,
            shop_name: hashmap.get(SHOP_NAME_FIELD).cloned().unwrap_or_default(),
            shop_country: hashmap.get(SHOP_COUNTRY_FIELD).cloned().unwrap_or_default(),
            shop_email: hashmap.get(EMAIL_FIELD).cloned().unwrap_or_default(),
            shop_domain: hashmap.get(SHOP_DOMAIN_FIELD).cloned().unwrap_or_default(),
            key: hashmap.get(KEY_FIELD).cloned().unwrap_or_default(),
        })
    }
}
