use reqwest::Error;
use chrono::Utc;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct BtcPrice {
    price: String,
    pub time: String,
}

#[derive(Deserialize, Debug)]
pub struct TradeResponse {
    tick: TickData,
}

#[derive(Deserialize, Debug)]
pub struct TickData {
    data: Vec<TradeData>,
}

#[derive(Deserialize, Debug)]
pub struct TradeData {
    price: f64,
}

pub async fn huobi_btc_price() -> Result<BtcPrice, Error> {
    let symbol = "btcusdt";
    let ticker_url = format!("https://api.huobi.pro/market/trade?symbol={}", symbol);
    let response: TradeResponse = reqwest::get(&ticker_url).await?.json().await?;
    let current_time = Utc::now().naive_utc();

    let most_recent_trade = response.tick.data.get(0);

    Ok(BtcPrice {
        price: most_recent_trade.as_ref().unwrap().price.to_string(),
        time: current_time.to_string(),
    })
}
