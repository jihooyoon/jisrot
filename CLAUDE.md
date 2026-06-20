# CLAUDE.md — AI Agent Instructions for jisrot

This file is for AI coding agents (Claude Code, Codex, Copilot, Cursor, etc.). It describes architecture, conventions, and mechanisms so any agent can correctly navigate and modify this codebase.

---

## Project Identity

- **Crate**: `jisrot` v0.1.1
- **Purpose**: Shopify app event history CSV analyzer → structured JSON stats
- **Rust edition**: 2024 (requires stable 1.85+)
- **GUI framework**: eframe/egui 0.33 (immediate mode, with persistence feature)
- **Binary entry**: `src/main.rs` → `app_egui::run()`

---

## File Map — what each file does

| File | Role | Editable? |
|------|------|-----------|
| `src/main.rs` | Binary entry point. `#![windows_subsystem = "windows"]` suppresses console. Calls `app_egui::run()`. | Yes |
| `src/main-cli.rs` | Broken CLI prototype. References non-existent `mod modal;`. **DO NOT EDIT — will not compile.** | No |
| `src/app_egui.rs` | Full egui GUI. `QuickGUIApp` struct holds all state. Combo boxes, file pickers, analyze button. | Yes |
| `src/analyzing.rs` | Core analysis engine. Two phases: `build_base_data()` → `analyze_details()`. Orchestrated by `analyze_from_gui()`. | Yes |
| `src/data_io.rs` | All file I/O. CSV reading (`read_events_from_csv`), JSON read/write for definitions and output. | Yes |
| `src/models/data_model.rs` | All data structs: `AppEvent`, `Merchant`, `MerchantList`, `TotalStats`, `PricingDefs`, `PricingUnit`, `ExcludingDef`, `BillingCycle`, `DetailedSubscriptionStats`, `SubscriptionStatsCounter`. | Yes |
| `src/models/ui_model.rs` | `UiOption` type — combo box value holder using `Cow<'static, str>`. | Yes |
| `src/definitions/common.rs` | Domain string constants — event type strings, CSV field names, date formats. | Yes |
| `src/definitions/strings.rs` | UI labels, error/success messages, built-in `UiOption` constants. | Yes |
| `src/definitions/default_ms_excluding_def.rs` | Embedded JSON string for Magestore exclusion definition. | Yes |
| `src/definitions/default_ms_pricing_def.rs` | Embedded JSON strings for SBM Barcode and SPOP Order Printer pricing definitions. | Yes |
| `build.rs` | Build script. On Windows: generates multi-res `.ico` from `ass/icon/icon256.png`, embeds via `winres`. On macOS: prints note about `cargo-bundle`. | Yes |

---

## Architecture — Data Flow

```
CSV file(s) on disk
  │
  ▼
data_io::read_events_from_csv()
  ├─ csv::Reader reads headers → builds IndexMap<String, String> per row
  ├─ AppEvent::from_indexmap() parses typed fields from the map
  └─ Returns Vec<AppEvent> sorted by time
  │
  ▼
analyzing::build_base_data(vec_of_events, excluding_regex)
  ├─ For each event: skip if excluding_check_data matches excluding_regex
  ├─ Match event string against INSTALLED_STRING, UNINSTALLED_STRING, etc.
  ├─ For subscriptions/one-time: match details against pricing plan regexes
  ├─ Count everything into Merchant structs inside MerchantList
  └─ Returns MerchantList (IndexMap<shop_domain, Merchant>)
  │
  ▼
analyzing::analyze_details(merchant_list, pricing_defs, case_sensitive)
  ├─ Per merchant: compute installed_status, subscription_status
  ├─ Find last_new_sub_plan (scan subscriptions in reverse)
  ├─ Find first_canceled_sub_plan (scan subscriptions forward)
  ├─ Compute aggregate churn_rate, sub_growth, paid_growth
  └─ Enriches MerchantList in-place, returns TotalStats
  │
  ▼
data_io::write_total_stats_to_json()  → Output/total_stats_<from>_<to>.json
data_io::write_merchant_data_to_json() → Output/merchant_data_<from>_<to>.json  (debug only)
data_io::write_app_event_list_to_json() → Output/app_event_list_<from>_<to>.json (debug only)
```

---

## Key Data Structures

### `AppEvent` (`models/data_model.rs`)
One CSV row, constructed via `AppEvent::from_indexmap(source: &IndexMap<String, String>, excluding_check_field: &str)`.
- `time: Option<NaiveDateTime>` — parsed from "Date" field (tries NaiveDateTime first, falls back to NaiveDate→midnight)
- `event: String` — the event type string
- `details: String` — matched against pricing plan regexes
- `billing_on: Option<NaiveDateTime>` — from "Billing on" field
- `shop_domain: String` — **primary key** for merchant grouping
- `excluding_check_data: String` — value of the CSV column named by `ExcludingDef.excluding_field`

### `Merchant` (`models/data_model.rs`)
Per-shop aggregation. Key fields:
- Counters: `installed_count`, `uninstalled_count`, `store_closed_count`, `store_reopened_count`, `one_time_count`, `new_sub_count`, `canceled_sub_count`
- Vectors: `installing_events`, `subscription_events`, `one_time_events`
- Status: `installed_status: String`, `subscription_status: String`
- Plan tracking: `last_new_sub_plan: Option<PricingUnit>`, `first_canceled_sub_plan: Option<PricingUnit>`
- Billing cycle: `last_new_sub_billing_cycle: Option<BillingCycle>`, `first_canceled_sub_billing_cycle: Option<BillingCycle>`

### `MerchantList` (`models/data_model.rs`)
`IndexMap<String, Merchant>` keyed by `shop_domain`. Has `time_from`/`time_to: Option<NaiveDateTime>`. Merchants are upserted — if a shop_domain already exists, it's updated; otherwise inserted.

### `TotalStats` (`models/data_model.rs`)
Aggregate output. Contains counters, derived metrics, and detailed breakdowns:
- `one_time_details: IndexMap<String, u32>` — per-plan one-time purchase counts
- `sub_stats_details: DetailedSubscriptionStats` — 5 counters (new/canceled/growth/all_new/all_canceled), each with `monthly_counts` and `yearly_counts` keyed by plan code

### `PricingDefs` / `PricingUnit` / `ExcludingDef` (`models/data_model.rs`)
- `PricingDefs { subscriptions: Vec<PricingUnit>, one_times: Vec<PricingUnit> }`
- `PricingUnit { code, name, regex_pattern, price, currency }`
- `ExcludingDef { excluding_field, excluding_pattern }`

### `UiOption` (`models/ui_model.rs`)
```rust
struct UiOption {
    value: Cow<'static, str>,   // the internal value used in logic
    text: Cow<'static, str>,    // the display text in combo box
}
```
Uses `Cow` so built-in options use `&'static str` (zero allocation) while custom options can use `String`.

---

## Coding Conventions — MUST FOLLOW

### 1. Getters/setters via `getset` crate
All data structs use `#[derive(Getters, Setters, MutGetters)]`. Fields that should NOT have public accessors use `#[getset(skip)]`. When adding a field, explicitly decide whether it needs getter/setter/mut_getter.

### 2. `IndexMap`, never `HashMap`
`IndexMap` preserves insertion order → deterministic JSON output. Always use `IndexMap` for new map fields that will be serialized. Import from `indexmap::IndexMap`.

### 3. Two-phase analysis boundary
- **`build_base_data()`**: only simple counting. Add new counters here.
- **`analyze_details()`**: only derived/computed metrics. Add new derived logic here.
- **`analyze_from_gui()`**: only orchestration (resolve defs, loop files, call phases, write output).
- Do NOT mix counting into `analyze_details()` or derivation into `build_base_data()`.

### 4. Regex handling
- The `case_sensitive_regex: bool` flag flows: `QuickGUIApp` → `analyze_from_gui()` → `analyze_details()`
- When `false` (default): lowercase BOTH the pattern and input before matching
- When `true`: match as-is
- The exclusion filter in `build_base_data()` also respects this flag

### 5. Event categorization pattern
All event type matching in `build_base_data()` uses the constants from `definitions/common.rs`:
```rust
if event_contain_check(&app_event.event, INSTALLED_STRING) { ... continue; }
if event_contain_check(&app_event.event, UNINSTALLED_STRING) { ... continue; }
// etc.
```
Each block ends with `continue` to skip further matching. The structure is a chain of `if/continue`, NOT `if-else if`.

### 6. Date/time parsing
`AppEvent::from_indexmap()` tries `NaiveDateTime::parse_from_str` first, then `NaiveDate::parse_from_str` (assumes midnight). Date format constants live in `definitions/common.rs`.

### 7. Adding a new built-in definition
1. Add JSON string constant in `definitions/default_ms_*.rs`
2. Add `UiOption` constant in `definitions/strings.rs` under the `ui` module
3. Wire into the match in `app_egui.rs` where built-in definitions are resolved

### 8. Adding a new event type
1. Add constant in `definitions/common.rs`
2. Add counter field to `Merchant` in `models/data_model.rs`
3. Add matching `if` block in `analyzing.rs` → `build_base_data()`
4. Add counter field to `TotalStats`, initialize in constructor
5. Update `analyze_details()` if needed

### 9. Adding a new GUI control
1. Add field to `QuickGUIApp` struct in `app_egui.rs`
2. Add `#[cfg_attr(feature = "persistence", serde(default))]` if it should persist
3. Add UI widget in the `update()` method
4. Pass the value through `analyze_from_gui()` parameters if it affects analysis

### 10. Error handling
Uses `anyhow::Result` throughout. Propagate with `?`, wrap context with `.context()` or `anyhow!()`.

---

## The `"reset"` Bug

In `src/main.rs`:
```rust
if args[0] == "reset" {  // BUG: args[0] is always the program path
```
Should be `args.get(1)` or `args[1]`. The reset feature is currently unreachable. Fix this if you need the reset functionality.

---

## GUI Details

### `QuickGUIApp` state fields
- `debug_mode: bool`
- `case_sensitive_regex: bool`
- `event_history_file_list: Option<Vec<PathBuf>>`
- `selected_pricing_defs_option: UiOption`
- `pricing_defs_file: Option<PathBuf>`
- `selected_excluding_defs_option: UiOption`
- `excluding_defs_file: Option<PathBuf>`

### Window config
- Size: 400×120 px, non-resizable
- Position: centered at (400, 200)
- `persist_window: false`
- File dialogs via `rfd::FileDialog`
- Message dialogs via `rfd::MessageDialog`

### Persistence
eframe's `persistence` feature stores state as key-value pairs. Use `"reset"` arg to clear (but see bug above).

---

## Build System

### `build.rs` — Windows icon generation
1. Reads `ass/icon/icon256.png`
2. Resizes to 16, 32, 48, 64, 128, 256 px (Lanczos3)
3. Assembles into `ass/icon/icon.ico`
4. Embeds via `winres`

On macOS: just prints a note about `cargo-bundle`.

### `Cargo.toml` key sections
- `[package.metadata.bundle]` — for `cargo-bundle` macOS bundling
- Build-dependencies: `image`, `ico`, `winres` (Windows only), `cargo-bundle`

### Code signing (Windows)
`sign_code_win.ps1` and `signtool.exe` are for post-build signing. Not part of the build process.

---

## Known Issues Summary

1. `main.rs` reset bug: `args[0]` → should be `args[1]`
2. `main-cli.rs`: broken, references missing module
3. No tests anywhere
4. Unsigned binary triggers Windows Defender false positive
