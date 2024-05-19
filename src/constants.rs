use std::collections::HashMap;
use std::fmt;
use once_cell::sync::Lazy;
use clap::ValueEnum;
use serde::{Serialize, Deserialize};

pub const SPOT_API_BASE_URL: &str = "https://api.binance.com/api/v3/";
pub const USD_FUTURES_API_BASE_URL: &str = "https://fapi.binance.com/fapi/v1/";
pub const COIN_FUTURES_API_BASE_URL: &str = "https://dapi.binance.com/dapi/v1/";
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Hash)]
pub enum MARKET {
    Spot,
    UsdFutures,
    CoinFutures,
}

impl fmt::Display for MARKET {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub static MARKET_BASE_URL: Lazy<HashMap<MARKET, &'static str>> = Lazy::new(|| {
    [
        (MARKET::Spot, SPOT_API_BASE_URL),
        (MARKET::UsdFutures, USD_FUTURES_API_BASE_URL),
        (MARKET::CoinFutures, COIN_FUTURES_API_BASE_URL),
    ].iter().cloned().collect()
});
// pub const CONTRACT_TYPES: [&str; 2] = ["PERPETUAL", "CURRENT_QUARTER"];
// pub enum ContractType {
//     Perpetual,
//     CurrentQuarter,
// }
// pub const INTERVALS: [&str; 15] = [
//     "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "1w", "1M"
// ];

pub const HEADERS: [&str; 11] = [
    "Open_Time",
    "Open",
    "High",
    "Low",
    "Close",
    "Volume",
    "Close",
    "Quote_Asset_Volume",
    "Number_Of_Trades",
    "Taker_Buy_Base_Asset_Volume",
    "Taker_Buy_Quote_Asset_Volume",
];
pub const KLINE_LIMIT: usize = 1000;