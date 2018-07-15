extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate rust_decimal;

use chrono::prelude::*;
use rust_decimal::Decimal;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct ClosedTrade {
  net_profit: i32,
  close_price: Decimal,
  closed_at: DateTime<Utc>,
  close_id: String,
  open_id: String,
  comment: String,
  commission: i32,
  open_price: Decimal,
  label: String,
  opened_at: DateTime<Utc>,
  swap: i32,
  trade_type: String,
  volume: Decimal,
  magic_number: i32,
  trading_symbol: TradingSymbol,
  updated_at: DateTime<Utc>,
  inserted_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
struct TradingSymbol {
  digits: u8,
  minimum_volume: String,
  symbol: String,
  volume: String,
}

#[derive(Serialize, Deserialize)]
struct Transfer {
  amount: i32,
  closed_at: DateTime<Utc>,
  investor_account_id: String,
  process_at: DateTime<Utc>,
  transaction_type: String,
}

#[derive(Serialize, Deserialize)]
struct Rebate {
  high_water_mark: String,
  high_water_mark_type: String,
  process_at: DateTime<Utc>,
  proportion: Decimal,
  rebate_splits: Vec<RebateSplit>,
  start_at: DateTime<Utc>,
  trade_type: String,
}

#[derive(Serialize, Deserialize)]
struct RebateSplit {
  investor_account_id: String,
  proportion: Decimal,
}

#[derive(Serialize, Deserialize)]
enum Event {
  ClosedTrade,
  Rebate,
  Transfer,
}

fn main() {
  let path = Path::new("method_events.json");
  let mut file = File::open(&path).expect("Failed to open json fixture data");
  let mut json_string_events = String::new();
  file
    .read_to_string(&mut json_string_events)
    .expect("error in copying file content to string");

  let event: Vec<serde_json::Value> =
    serde_json::from_str(&json_string_events).expect("json parse error");
  println!("{:?}", event);
}
