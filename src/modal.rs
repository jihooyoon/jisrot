use std::collections::HashMap;
use chrono::{NaiveDateTime, NaiveDate};
use crate::definitions::common::*;
use serde::Deserialize;

pub struct MerchantData {
    checked: bool,
    installed_count: u32,
    uninstalled_count: u32,
    store_closed_count: u32,
    store_reopened_count: u32,
    installing_events: Vec<AppEvent>,
    subscription_activated_count: u32,
    subscription_canceled_count: u32,
    subscription_events: Vec<AppEvent>,
    one_time_count: u32,
    one_time_details: HashMap<String, u32>,
    one_time_events: Vec<AppEvent>,
    installed_status: String,
    subscription_status: String,
}

pub struct MerchantDataList {
    start_time: String,
    end_time: String,
    merchants: HashMap<String, MerchantData>,
}

#[derive(Debug, Deserialize)]
pub struct ExcludingDef {
    excluding_field: String,
    excluding_pattern: String
}

#[derive(Debug, Deserialize)]
pub struct PricingDefs {
    subscriptions: Vec<PricingUnit>,
    one_times: Vec<PricingUnit>,
}

#[derive(Debug, Deserialize)]
pub struct PricingUnit {
    code: String,
    name: String,
    regex_pattern: String,
    price: f64,
    currency: String,
}

#[derive(Debug)]
pub struct AppEvent {
    time: Option<NaiveDateTime>,
    event: String,
    details: String,
    billing_on: Option<NaiveDateTime>,
    shop_name: String,
    shop_country: String,
    shop_email: String,
    shop_domain: String,
    key: String    
}

impl AppEvent  {
    
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

    fn parse_time(data_hash: &HashMap<String, String>, data_field: &str, pattern: &str) -> Result<NaiveDateTime, String> {
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
                                Err(format!("Invalid date format for field {}: {}", data_field, time_string))
                            }
                        },
                        Err(e) => {
                            eprintln!("Date Parse error: {:?}", e);
                            Err(format!("Invalid date format for field {}: {}", data_field, time_string))
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