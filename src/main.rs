use std::fs;
use std::path::Path;

use chrono::{Duration, Utc};
use clap::{Parser, Subcommand};

mod constants;
mod types;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    // #[arg(short, long, default_value = "spot")]
    // market: constants::MARKET,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all symbols of a market
    ListSymbols {
        #[arg(short, long, default_value = "spot")]
        market: constants::MARKET,
    },
    /// download klines of a market, either all symbols or specific symbols
    GetKlines {
        /// market to download klines from
        #[arg(short, long, default_value = "spot")]
        market: constants::MARKET,

        /// If true, download klines of all symbols, otherwise download klines of specific symbols
        #[arg(long, default_value = "false")]
        all: bool,

        /// Interval of the klines, supports "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "1w", "1M"
        #[arg(short, long, default_value = "1h")]
        interval: String,

        /// UTC Start Time, supports both unix timestamp in milliseconds or date string in this format: "2022-01-01 00:00:00" . Default is 30 days ago of the end time
        #[arg(long)]
        start_time: Option<String>,

        /// UTC End Time, supports both unix timestamp in milliseconds or date string in this format: "2022-01-01 00:00:00" . Default is now()
        #[arg(long)]
        end_time: Option<String>,
        
        /// Director for saving the downloaded klines, default is current directory
        #[arg(long, short)]
        director: Option<String>,

        symbols: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    // println!("{:?}", cli);
    match &cli.command {
        Commands::ListSymbols { market } => {
            let api_base_url = constants::MARKET_BASE_URL.get(market).unwrap().to_string();
            let symbols = utils::get_trading_symbols(api_base_url);
            println!("{:?}", symbols);
        }
        Commands::GetKlines { market, all, interval, start_time, end_time, symbols, director } => {
            if !all && symbols.len() == 0 {
                println!("Please provide symbols to download klines from or use --all to download klines of all symbols.");
                return;
            }
            let d = director.clone().unwrap_or(".".to_string());
            let p = Path::new(&d);
            if !p.exists() {
                println!("Director {} does not exist.", fs::canonicalize(p).unwrap().to_str().unwrap());
                return;
            }
            let end_datetime = match end_time {
                Some(end_time) => utils::parse_date_time(end_time).unwrap(),
                None => Utc::now(),
            };
            let start_datetime = match start_time {
                Some(start_time) => utils::parse_date_time(start_time).unwrap(),
                None => end_datetime - Duration::days(30)
            };
            let api_base_url = constants::MARKET_BASE_URL.get(market).unwrap().to_string();
            println!("Start time: {}, End time: {}, saving files to: {}", start_datetime, end_datetime, fs::canonicalize(p).unwrap().to_str().unwrap());
            let fetch_props = types::FetchProps {
                api_base_url: api_base_url.clone(),
                market: *market,
                contract_type: "".to_string(),
                symbol: "".to_string(),
                interval: (*interval.clone()).to_string(),
                start_time: start_datetime.timestamp_millis(),
                end_time: end_datetime.timestamp_millis(),
                director: d
            };
            if *all {
                let symbols = utils::get_trading_symbols(api_base_url);
                utils::get_historical_candlesticks_for_symbols(fetch_props, symbols);
            } else {
                utils::get_historical_candlesticks_for_symbols(fetch_props, (*symbols).clone());
            }
        }
    }
}
