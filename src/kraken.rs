use rocket::serde::json::Json;
use rocket::http::Status;
use reqwest::Error;
use chrono::{NaiveDateTime, Utc, TimeZone};
use serde::{Serialize, Deserialize};
use rocket::get;
use rocket::routes;
use tokio::time::{self, Duration};

#[derive(Serialize, Debug)]
pub struct BtcPrice {
    pub price: String,
    pub time: String,
}

#[derive(Deserialize, Debug)]
pub struct KrakenResponse {
    error: Vec<String>,
    result: std::collections::HashMap<String, KrakenTicker>,
}

#[derive(Deserialize, Debug)]
struct KrakenTicker {
    c: Vec<String>,
}

#[derive(Serialize)]
pub struct Trade {
    price: String,
    volume: String,
    datetime: String,
    action: String,
}

pub enum KrakenError {
    RequestError(reqwest::Error),
    InternalServerError(String),
}

impl From<reqwest::Error> for KrakenError {
    fn from(err: reqwest::Error) -> Self {
        KrakenError::RequestError(err)
    }
}

pub async fn kraken_btc_trades() -> Result<Vec<Trade>, Box<dyn std::error::Error>> {
    let url = "https://api.kraken.com/0/public/Trades?pair=XBTUSD";

    let resp: serde_json::Value = reqwest::get(url).await?.json().await?;
    
    let raw_trades: Vec<serde_json::Value> = serde_json::from_value(resp["result"]["XXBTZUSD"].clone())?;
    let trades: Vec<Trade> = raw_trades.iter()
        .take(20)
        .map(|trade| {
            let price = trade[0].as_str().unwrap_or_default().to_string();
            let volume = trade[1].as_str().unwrap_or_default().to_string();
            let timestamp = trade[2].as_f64().unwrap_or_default() as i64;
            
            let naive_datetime = NaiveDateTime::from_timestamp(timestamp, 0);
            let datetime: String = Utc.from_local_datetime(&naive_datetime).single().unwrap().to_string();

            let action = match trade[3].as_str().unwrap_or_default() {
                "b" => "buy".to_string(),
                "s" => "sell".to_string(),
                _ => "unknown".to_string(),
            };
            
            Trade { price, volume, datetime, action }
        })
        .collect();
    
    Ok(trades)
}

pub async fn kraken_btc_price() -> Result<BtcPrice, KrakenError> {
    let symbol = "XXBTZUSD";
    let ticker_url = format!("https://api.kraken.com/0/public/Ticker?pair={}", symbol);
    let response: KrakenResponse = reqwest::get(&ticker_url).await?.json().await?;

    if !response.error.is_empty() {
        return Err(KrakenError::InternalServerError(response.error.join(",")));
    }

    let ticker_data = response.result.get(symbol).ok_or_else(|| {
        KrakenError::InternalServerError("Symbol not found in Kraken response".to_string())
    })?;

    let current_time = Utc::now().naive_utc();

    let price = ticker_data.c.get(0).ok_or_else(|| {
        KrakenError::InternalServerError("Price not found in Kraken ticker data".to_string())
    })?.clone();

    Ok(BtcPrice {
        price,
        time: current_time.to_string(),
    })
}
