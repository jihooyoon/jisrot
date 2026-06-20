# Ji's Shopify Researching Operation Tools

A tiny event analyzing tool for Shopify App — processes CSV event history exports from the Shopify Partner dashboard and produces structured statistics about merchant behavior.

---

## What it does

Given one or more CSV event history files (exported from Shopify Partner dashboard), the tool:

1. **Parses** every row into a structured event
2. **Filters out** excluded shops (configurable regex, e.g. your own test store)
3. **Categorizes** events into: installed, uninstalled, store closed, store reopened, one-time charge, subscription activated, subscription canceled
4. **Groups by merchant** (shop domain) and counts everything
5. **Detects pricing plans** by matching event details against regex patterns you define
6. **Detects billing cycles** (monthly vs yearly) from event details
7. **Computes derived metrics**: churn rate, merchant growth, subscription growth, paid growth — broken down by plan and billing cycle
8. **Outputs** results as pretty-printed JSON

### Output files

| File | Contents |
|------|----------|
| `Output/total_stats_<from>_<to>.json` | Aggregate stats (always written) |
| `Output/merchant_data_<from>_<to>.json` | Per-merchant breakdown (debug mode only) |
| `Output/app_event_list_<from>_<to>.json` | Parsed events (debug mode only) |

---

## Two-phase analysis algorithm

### Phase 1: Counting
Events are processed **in chronological order**. Each event is matched against known event type strings (installed, uninstalled, etc.) and counted per merchant. Events from excluded shops are skipped entirely.

### Phase 2: Derived metrics
After all events are counted, per-merchant status is computed:
- **Installed status**: net delta of install+reopen minus uninstall+close. A merchant whose first event was "Uninstalled" is labeled "old uninstalled" (was already gone before the data range).
- **Subscription status**: net delta of activated minus canceled subscriptions.
- **Plan & billing cycle**: scans subscription events to find the latest activated plan and earliest canceled plan. Billing cycle is yearly if event details mention "Year", otherwise monthly.
- **Aggregate metrics**: total churn rate, subscription growth, paid growth — computed from the final counts.

---

## How to use

### GUI mode
```bash
cargo run                    # Launch the GUI
cargo run -- reset           # Reset saved settings, then launch
```

1. Select a **pricing definition** from the dropdown (or choose "Custom" and browse a JSON file)
2. Select an **excluding definition** (or choose "Custom")
3. Click **"Browse event history file..."** to pick one or more CSV files
4. Optionally enable **Debug mode** for extra output files
5. Click **Analyze!**

Results appear in `Output/` under the same directory as the first CSV file.

### Custom definition files

See `sample_definitions_json/` for examples:

**Excluding definition** (`ms_excluding_def.json`):
```json
{
  "excluding_field": "Shop email",
  "excluding_pattern": "magestore"
}
```
Events where the specified field matches the regex are skipped. Typically used to filter out your own test store.

**Pricing definition** (`sbm_pricing_def.json`):
```json
{
  "subscriptions": [
    { "code": "standard", "name": "Standard", "regex_pattern": "Standard", "price": 7.99, "currency": "USD" }
  ],
  "one_times": [
    { "code": "2000_labels", "name": "2000 Labels", "regex_pattern": "2000 Labels", "price": 11.99, "currency": "USD" }
  ]
}
```
Each plan has a `regex_pattern` matched against the event details field to identify which plan an event belongs to.

---

## Build
```bash
cargo build --release
```

Requires Rust stable 1.85+ (edition 2024).

---

## Note on Windows Defender
The binary is unsigned (code signing requires a paid developer account). Windows Defender may flag it as a virus — this is a **false positive**. Verify with VirusTotal or build from source.

---

## License
AGPLv3
