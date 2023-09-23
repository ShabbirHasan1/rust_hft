use reqwest::Error;
use chrono::NaiveDateTime;
use chrono::Utc;
use chrono::TimeZone;
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

#[derive(Deserialize, Debug)]
struct Trade {
    id: f64,
    ts: u64,
    price: f64,
    amount: f64,
    direction: String,
}

#[derive(Deserialize, Debug)]
struct Tick {
    data: Vec<Trade>,
}

#[derive(Deserialize, Debug)]
struct Trades {
    tick: Tick,
}

#[derive(Serialize, Debug)]
pub struct BtcTrade {
    pub timestamp: String,
    pub price: String,
    pub amount: String,
    pub direction: String,
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

pub async fn huobi_btc_trades() -> Result<Vec<BtcTrade>, Error> {
    let symbol = "btcusdt";
    let trades_url = format!("https://api.huobi.pro/market/trade?symbol={}", symbol);
    let response: Trades = reqwest::get(&trades_url).await?.json().await?;

    if response.tick.data.is_empty() {
        return Ok(vec![]);
    }

    let trades: Vec<BtcTrade> = response.tick.data.iter().map(|trade| {
        BtcTrade {
            timestamp: Utc.timestamp_opt((trade.ts / 1000) as i64, 0).single().unwrap_or(Utc.timestamp(0, 0)).to_string(),
            price: trade.price.to_string(),
            amount: trade.amount.to_string(),
            direction: trade.direction.clone(),
        }
    }).collect();

    Ok(trades)
}
