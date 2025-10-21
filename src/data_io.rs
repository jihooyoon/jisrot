use anyhow::{Result, anyhow};
use std::path::PathBuf;

use crate::definitions::common::*;
use crate::models::data_model::*;
use indexmap::IndexMap;

pub fn read_events_from_csv(
    source_file: &PathBuf,
    excluding_check_field: &str,
) -> anyhow::Result<Vec<AppEvent>> {
    let mut rdr = csv::Reader::from_path(source_file)?;
    let headers: Vec<String> = rdr.headers()?.iter().map(|h| h.to_string()).collect();

    let mut app_event_list: Vec<AppEvent> = Vec::new();

    for record in rdr.records() {
        let record_hash: IndexMap<String, String> = record?
            .iter()
            .zip(headers.iter())
            .map(|(value, header)| (header.to_string(), value.to_string()))
            .collect();

        match AppEvent::from_indexmap(&record_hash, excluding_check_field) {
            Ok(app_event) => app_event_list.push(app_event),
            Err(e) => return Err(anyhow!("Error parsing record: {}", e)),
        }
    }

    Ok(app_event_list)
}

pub fn read_pricing_def_from_json(file_in: &PathBuf) -> anyhow::Result<PricingDefs> {
    let file = std::fs::File::open(file_in)?;
    let reader = std::io::BufReader::new(file);

    let result = serde_json::from_reader(reader)?;
    Ok(result)
}

pub fn read_pricing_def_from_json_str(json_str: &str) -> anyhow::Result<PricingDefs> {
    Ok(serde_json::from_str(json_str)?)
}

pub fn read_excluding_def_from_json(file_in: &PathBuf) -> anyhow::Result<ExcludingDef> {
    let file = std::fs::File::open(file_in)?;
    let reader = std::io::BufReader::new(file);

    let result = serde_json::from_reader(reader)?;
    Ok(result)
}

pub fn read_excluding_def_from_json_str(json_str: &str) -> anyhow::Result<ExcludingDef> {
    Ok(serde_json::from_str(json_str)?)
}

pub fn write_app_event_list_to_json(
    file_out: &PathBuf,
    app_event_list: &Vec<AppEvent>,
) -> anyhow::Result<()> {
    if let Some(parent) = file_out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = std::fs::File::create(file_out)?;
    let writer = std::io::BufWriter::new(file);

    serde_json::to_writer_pretty(writer, app_event_list)?;
    Ok(())
}

pub fn write_total_stats_to_json(
    file_out: &PathBuf,
    total_stats: &TotalStats,
) -> anyhow::Result<()> {
    if let Some(parent) = file_out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = std::fs::File::create(file_out)?;
    let writer = std::io::BufWriter::new(file);

    serde_json::to_writer_pretty(writer, total_stats)?;
    Ok(())
}

pub fn write_merchant_data_to_json(
    file_out: &PathBuf,
    merchant_data_list: &MerchantDataList,
) -> anyhow::Result<()> {
    if let Some(parent) = file_out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = std::fs::File::create(file_out)?;
    let writer = std::io::BufWriter::new(file);

    serde_json::to_writer_pretty(writer, merchant_data_list)?;
    Ok(())
}
