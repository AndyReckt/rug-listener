use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeData {
    #[serde(rename = "type")]
    pub trade_type: String,
    pub username: String,
    #[serde(rename = "userImage")]
    pub user_image: String,
    pub amount: f64,
    #[serde(rename = "coinSymbol")]
    pub coin_symbol: String,
    #[serde(rename = "coinName")]
    pub coin_name: String,
    #[serde(rename = "coinIcon")]
    pub coin_icon: String,
    #[serde(rename = "totalValue")]
    pub total_value: f64,
    pub price: f64,
    pub timestamp: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceUpdateData {
    #[serde(rename = "coinSymbol")]
    pub coin_symbol: String,
    #[serde(rename = "currentPrice")]
    pub current_price: f64,
    #[serde(rename = "marketCap")]
    pub market_cap: f64,
    #[serde(rename = "change24h")]
    pub change_24h: f64,
    #[serde(rename = "volume24h")]
    pub volume_24h: f64,
    #[serde(rename = "poolCoinAmount")]
    pub pool_coin_amount: f64,
    #[serde(rename = "poolBaseCurrencyAmount")]
    pub pool_base_currency_amount: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WSMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub data: TradeData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceWSMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(rename = "coinSymbol")]
    pub coin_symbol: String,
    #[serde(rename = "currentPrice")]
    pub current_price: f64,
    #[serde(rename = "marketCap")]
    pub market_cap: f64,
    #[serde(rename = "change24h")]
    pub change_24h: f64,
    #[serde(rename = "volume24h")]
    pub volume_24h: f64,
    #[serde(rename = "poolCoinAmount")]
    pub pool_coin_amount: f64,
    #[serde(rename = "poolBaseCurrencyAmount")]
    pub pool_base_currency_amount: f64,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub msg_type: String,
    pub data: TradeData,
    pub received_at: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct PriceUpdate {
    pub coin_symbol: String,
    pub current_price: f64,
    pub market_cap: f64,
    pub change_24h: f64,
    pub volume_24h: f64,
    pub pool_coin_amount: f64,
    pub pool_base_currency_amount: f64,
    pub received_at: DateTime<Local>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TradeFilter {
    All,
    Large,
}

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    CoinFilter,
    TraderFilter,
    CoinSelection,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppPage {
    Trades,
    PriceTracker,
}