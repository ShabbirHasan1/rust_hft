use rocket::serde::json::Json;
use rocket::http::Status;
use reqwest::Error;
use chrono::Utc;
use chrono::NaiveDateTime;
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

pub enum KrakenError {
    RequestError(reqwest::Error),
    InternalServerError(String),
}

impl From<reqwest::Error> for KrakenError {
    fn from(err: reqwest::Error) -> Self {
        KrakenError::RequestError(err)
    }
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
