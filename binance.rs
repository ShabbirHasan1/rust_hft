use reqwest::Error;
use chrono::Utc;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct TickerPrice {
    symbol: String,
    price: String,
}

#[derive(Serialize, Debug)]
pub struct BtcPrice {
    pub price: String,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecentTrade {    
    id: i64,
    price: String,
    qty: String,
    time: i64,
}

#[derive(Serialize, Debug)]
pub struct BtcTrades {
    trades: Vec<RecentTrade>,
    time: String,
}

#[derive(Serialize, Debug)]
pub struct Bids {
    bids: Vec<Vec<String>>
}

#[derive(Serialize, Debug)]
pub struct Asks {
    asks: Vec<Vec<String>>
}

#[derive(Deserialize, Debug)]
pub struct OrderBook {
    lastUpdateId: i64,
    bids: Vec<Vec<String>>,
    asks: Vec<Vec<String>>,
}

const CLICKHOUSE_ENDPOINT: &str = "http://localhost:8123/";

pub async fn binance_btc_price() -> Result<BtcPrice, Error> {
    let symbol = "BTCUSDT";
    let ticker_url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    let ticker_response: TickerPrice = reqwest::get(&ticker_url).await?.json().await?;
    let current_time = Utc::now().naive_utc();

    Ok(BtcPrice {
        price: ticker_response.price,
        time: current_time.to_string(),
    })
}

pub async fn binance_btc_trades() -> Result<BtcTrades, Error> {
    let symbol = "BTCUSDT";
    let recent_trades_url = format!("https://api.binance.com/api/v3/trades?symbol={}&limit=20", symbol);
    let recent_trades_response: Vec<RecentTrade> = reqwest::get(&recent_trades_url).await?.json().await?;
    let current_time = Utc::now().naive_utc().to_string();

    Ok(BtcTrades {
        trades: recent_trades_response,
        time: current_time,
    })
}

pub async fn binance_order_book_data() -> Result<(Bids, Asks), Error> {
    let symbol = "BTCUSDT";

    let order_book_url = format!("https://api.binance.com/api/v3/depth?symbol={}&limit=10", symbol);
    let order_book_response: OrderBook = reqwest::get(&order_book_url).await?.json().await?;
    
    let bids_data = Bids {
        bids: order_book_response.bids,
    };

    let asks_data = Asks {
        asks: order_book_response.asks,
    };
   
    Ok((bids_data, asks_data))
}

pub async fn binance_price_clickhouse(timestamp: &NaiveDateTime, price: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let query = format!(
        "INSERT INTO btc_price (timestamp, price) VALUES ('{}', '{}')",
        timestamp.format("%Y-%m-%d %H:%M:%S"), 
        price
    );

    let response = client.post(CLICKHOUSE_ENDPOINT)
        .body(query)
        .send()
        .await?;

    if response.status().is_success() {
    } else {
        eprintln!("Error inserting price data: {}", response.status());
    }

    Ok(())
}