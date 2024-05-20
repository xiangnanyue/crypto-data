use std::collections::HashMap;
use std::path::{Path, PathBuf};
use chrono::{DateTime, NaiveDateTime, Utc};
use crate::constants;
use crate::constants::{HEADERS, KLINE_LIMIT};
use crate::types::{Candle, ExchangeInfo, FetchProps};
// transform above typescript code to rust:
pub fn generate_query_string(fetch_props: FetchProps) -> String {
    let is_spot_market_selected = match fetch_props.market {
        constants::MARKET::Spot => true,
        _ => false,
    };
    let start_time = fetch_props.start_time.to_string();
    let end_time = fetch_props.end_time.to_string();
    let mut query_data: HashMap<&str, &str> = [
        ("symbol", fetch_props.symbol.as_str()),
        ("interval", fetch_props.interval.as_str()),
        ("startTime", start_time.as_str()),
        ("endTime", end_time.as_str()),
        ("limit", "1000")
    ].iter().cloned().collect();

    if !is_spot_market_selected {
        query_data.insert("contractType", &fetch_props.contract_type);
    }
    return format!("klines?{}", serde_qs::to_string(&query_data).unwrap());
}

pub fn encode_candle(candles: Vec<Candle>) -> Vec<Vec<String>> {
    candles.iter().map(|e| vec![
        e.0.to_string(),
        e.1.clone(),
        e.2.clone(),
        e.3.clone(),
        e.4.clone(),
        e.5.clone(),
        e.6.to_string(),
        e.7.clone(),
        e.8.to_string(),
        e.9.clone(),
        e.10.clone(),
    ]).collect::<Vec<Vec<String>>>()
}

pub fn create_csv_file(fetch_props: FetchProps, data: Vec<Candle>, init_start_time: i64) -> Result<(), Box<dyn std::error::Error>> {
    let csv_content = encode_candle(data);
    let p = csv_file_path(&fetch_props, init_start_time);
    let mut wtr = csv::Writer::from_path(p).unwrap();
    wtr.write_record(HEADERS.to_vec()).unwrap();
    for candle in csv_content {
        wtr.write_record(candle).unwrap();
    }
    wtr.flush().unwrap();
    Ok(())
}

fn csv_file_path(fetch_props: &FetchProps, init_start_time: i64) -> PathBuf {
    let filename = format!(
        "{}_{}_{}_{}_{}.csv",
        fetch_props.symbol,
        fetch_props.market,
        fetch_props.interval,
        init_start_time,
        fetch_props.end_time
    );
    Path::join(Path::new(&fetch_props.director), filename.as_str())
}

// pub fn interval_to_milliseconds(interval: &str) -> u64 {
//     let seconds_per_unit = [
//         ("m", 60),
//         ("h", 60 * 60),
//         ("d", 24 * 60 * 60),
//         ("w", 7 * 24 * 60 * 60),
//         ("M", 7 * 24 * 60 * 60 * 3),
//     ].iter().cloned().collect::<HashMap<&str, u64>>();
//     let interval_number = interval[..interval.len() - 1].parse::<u64>().unwrap();
//     let interval_unit = &interval[interval.len() - 1..];
//     interval_number * seconds_per_unit[interval_unit] * 1000
// }

// pub fn contract_type_to_title_case(contract_type: &str) -> String {
//     contract_type
//         .to_lowercase()
//         .split('_')
//         .map(|x| x.chars().next().unwrap().to_uppercase().to_string() + &x[1..])
//         .collect::<Vec<String>>()
//         .join(" ")
// }

// pub fn contract_type_to_query_value(contract_type: &str) -> String {
//     contract_type.replace(" ", "_").to_uppercase()
// }

// pub fn calculate_request_length(fetch_props: FetchProps) -> u64 {
//     (fetch_props.end_time - fetch_props.start_time) / interval_to_milliseconds(&fetch_props.interval)
// }

// pub fn generate_max_date(time_stamp: i64) -> String {
//     let date = time_stamp - 86400000;
//     let date_str = chrono::NaiveDateTime::from_timestamp(date / 1000, 0).to_string();
//     date_str.split(' ').collect::<Vec<&str>>()[0].to_string()
// }
//
// pub fn generate_min_date(time_stamp: i64) -> String {
//     let date = time_stamp + 86400000;
//     let date_str = DateTime::from_timestamp(date / 1000, 0).unwrap().to_string();
//     date_str.split(' ').collect::<Vec<&str>>()[0].to_string()
// }


pub fn get_historical_candlesticks_for_symbols(fetch_props: FetchProps, symbols: Vec<String>) {
    let total = symbols.len();
    let pb = indicatif::ProgressBar::new(total as u64);
    for symbol in symbols {
        let props = FetchProps {
            api_base_url: fetch_props.api_base_url.clone(),
            market: fetch_props.market.clone(),
            contract_type: fetch_props.contract_type.clone(),
            symbol: symbol.clone(),
            interval: fetch_props.interval.clone(),
            start_time: fetch_props.start_time,
            end_time: fetch_props.end_time,
            director: fetch_props.director.clone(),
        };
        let p = csv_file_path(&props, fetch_props.start_time);
        if p.exists() {
            pb.println(format!("[+] {} already exists, skipping...", symbol));
            pb.inc(1);
            continue;
        }

        // Retry up to 3 times
        for _ in 0..3 {
            match get_historical_candlesticks(props.clone()) {
                Ok(_) => {
                    pb.println(format!("[+] finished {}", symbol));
                    pb.inc(1);
                    break;
                }
                Err(e) => {
                    eprintln!("Error fetching historical candlesticks for {}: {}. Retrying...", symbol, e);
                }
            }
        }
    }
    pb.finish_with_message("done");
}

fn get_historical_candlesticks(fetch_props: FetchProps) -> Result<(), Box<dyn std::error::Error>> {
    let candles: Vec<Candle> = vec![];
    let start_time = fetch_props.start_time;
    do_get_historical_candlesticks(fetch_props, candles, start_time)
}

fn do_get_historical_candlesticks(fetch_props: FetchProps, mut candles: Vec<Candle>, init_start_time: i64) -> Result<(), Box<dyn std::error::Error>>{
    if fetch_props.start_time > fetch_props.end_time {
        panic!("Start time cannot be greater than end time");
    }
    let url = format!("{}{}", fetch_props.api_base_url, generate_query_string(fetch_props.clone()));

    let res = reqwest::blocking::get(&url);
    match res {
        Ok(response) => {
            // rewrite this: handle error if re.json() is not ok
            let new_candles: Vec<Candle> = response.json().expect(format!("Can not get klines for {}", fetch_props.symbol).as_str());

            let is_finished = new_candles.len() < KLINE_LIMIT;
            // Merge the newly fetched candles with the existing ones

            if is_finished {
                candles.extend(new_candles);
                create_csv_file(fetch_props.clone(), candles, init_start_time)
            } else {
                let last_candle = new_candles.last().cloned().unwrap();
                candles.extend(new_candles);
                let last_candle_start_time = last_candle.0;
                candles.pop();
                // create_csv_file(fetch_props.clone(), new_candles);
                let fetch_props_clone = fetch_props.clone();
                do_get_historical_candlesticks(FetchProps {
                    api_base_url: fetch_props_clone.api_base_url,
                    market: fetch_props_clone.market,
                    contract_type: fetch_props_clone.contract_type,
                    symbol: fetch_props_clone.symbol,
                    interval: fetch_props_clone.interval,
                    start_time: last_candle_start_time,
                    end_time: fetch_props_clone.end_time,
                    director: fetch_props_clone.director,
                }, candles, init_start_time)
            }
        }
        Err(e) => {
            eprintln!("Error fetching historical candlesticks for {}: {}", fetch_props.symbol, e);
            Err(Box::new(e))
        }
        
    }

}

pub fn get_trading_symbols(api_base_url: String) -> Vec<String> {
    let url = format!("{}{}", api_base_url, "exchangeInfo");
    let res = reqwest::blocking::get(&url).unwrap();
    let exchange_info: ExchangeInfo = res.json().unwrap();
    let symbols = exchange_info.symbols;
    let symbols_list = symbols.iter()
        .filter(|x|
            match x.status.as_ref() {
                Some(v) => v == "TRADING",
                None => true,
            }
        )
        .map(|x| x.symbol.clone()).collect::<Vec<String>>();
    symbols_list
}

// translate to Rust:
pub fn parse_date_time(date_string: &str) -> Result<DateTime<Utc>, &'static str> {
    // Try to parse as Unix timestamp
    if let Ok(timestamp) = date_string.parse::<i64>() {
        if let Some(date) = DateTime::from_timestamp(timestamp / 1000, 0) {
            return Ok(date);
        }
    }

    // Try to parse as date string
    if let Ok(naive_datetime) = NaiveDateTime::parse_from_str(date_string, "%Y-%m-%d %H:%M:%S") {
        return Ok(naive_datetime.and_utc())
    }

    Err("Failed to parse date string")
}
#[cfg(test)]
mod tests {
    // use crate::constants::{INTERVALS, SPOT_API_BASE_URL};
    // use crate::types::FetchProps;
    // use crate::{constants, utils};
    //
    // #[test]
    // fn it_works() {
    //     let props = FetchProps {
    //         api_base_url: SPOT_API_BASE_URL.to_string(),
    //         market: constants::MARKET::Spot,
    //         contract_type: "".to_string(),
    //         symbol: "btcusdt".to_string(),
    //         interval: INTERVALS.get(1).unwrap().to_string(),
    //         start_time: 1715940401621,
    //         end_time: 1716000461621,
    //     };
    //     utils::get_historical_candlesticks(props);
    // }

    #[test]
    fn all_symbols() {
        // let props = FetchProps {
        //     api_base_url: SPOT_API_BASE_URL.to_string(),
        //     market: constants::MARKET::Spot,
        //     contract_type: "".to_string(),
        //     symbol: "".to_string(),
        //     interval: INTERVALS.get(1).unwrap().to_string(),
        //     start_time: 1715940401621,
        //     end_time: 17160004616210,
        // };
        // utils::get_historical_candlesticks_for_all_symbols(props);
        // let symbols = utils::get_trading_symbols(props);
        // // println!("{:?}", symbols);
        // for symbol in symbols {
        //     let props = FetchProps {
        //         api_base_url: SPOT_API_BASE_URL.to_string(),
        //         market: MARKETS.get(0).unwrap().to_string(),
        //         contract_type: "".to_string(),
        //         symbol,
        //         interval: INTERVALS.get(1).unwrap().to_string(),
        //         start_time: 1715501325370,
        //         end_time: 1716106137442,
        //     };
        //     utils::get_historical_candlesticks(props);
        // }
        // assert_eq!(symbols.len(), 1000);
    }
}