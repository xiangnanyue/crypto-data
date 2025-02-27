use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::constants::MARKET;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FetchProps {
    pub api_base_url: String,
    pub market: MARKET,
    pub contract_type: String,
    pub symbol: String,
    pub interval: String,
    pub start_time: i64,
    pub end_time: i64,
    pub director: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchPropsAction {
    pub type_: String,
    pub payload: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    pub exchange_filters: Option<Vec<Value>>,
    pub rate_limits: Option<Vec<RateLimit>>,
    pub server_time: Option<i64>,
    pub assets: Option<Vec<Asset>>,
    pub symbols: Vec<Symbol>,
    pub timezone: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset: String,
    pub margin_available: bool,
    pub auto_asset_exchange: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub interval: String,
    pub interval_num: i64,
    pub limit: i64,
    pub rate_limit_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub pair: Option<String>,
    pub contract_type: Option<String>,
    pub delivery_date: Option<i64>,
    pub onboard_date: Option<i64>,
    pub status: Option<String>,
    pub maint_margin_percent: Option<String>,
    pub required_margin_percent: Option<String>,
    pub base_asset: String,
    pub quote_asset: String,
    pub margin_asset: Option<String>,
    pub price_precision: Option<i64>,
    pub quantity_precision: Option<i64>,
    pub base_asset_precision: Option<i64>,
    pub quote_precision: Option<i64>,
    pub underlying_type: Option<String>,
    pub underlying_sub_type: Option<Vec<String>>,
    pub settle_plan: Option<i64>,
    pub trigger_protect: Option<String>,
    pub filters: Vec<Filter>,
    pub order_type: Option<Vec<String>>,
    pub time_in_force: Option<Vec<String>>,
    pub liquidation_fee: Option<String>,
    pub market_take_bound: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub filter_type: Option<String>,
    pub max_price: Option<String>,
    pub min_price: Option<String>,
    pub tick_size: Option<String>,
    pub max_qty: Option<String>,
    pub min_qty: Option<String>,
    pub step_size: Option<String>,
    pub limit: Option<i64>,
    pub notional: Option<String>,
    pub multiplier_up: Option<String>,
    pub multiplier_down: Option<String>,
    pub multiplier_decimal: Option<String>,
}

pub type Candle = (i64, String, String, String, String, String, u64, String, u64, String, String, String);
