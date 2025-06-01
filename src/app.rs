use crate::models::{AppPage, InputMode, PriceUpdate, Trade, TradeFilter};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub const MAX_TRADES: usize = 1000;
pub const MAX_PRICE_UPDATES: usize = 100;

#[derive(Debug)]
pub struct App {
    pub trades: Arc<Mutex<VecDeque<Trade>>>,
    pub price_updates: Arc<Mutex<VecDeque<PriceUpdate>>>,
    pub current_page: AppPage,
    pub trade_filter: TradeFilter,
    pub coin_filter: String,
    pub trader_filter: String,
    pub selected_tab: usize,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub scroll_offset: usize,
    pub tracked_coin: Option<String>,
    pub latest_price: Option<PriceUpdate>,
}

impl App {
    pub fn new(trades: Arc<Mutex<VecDeque<Trade>>>, price_updates: Arc<Mutex<VecDeque<PriceUpdate>>>) -> Self {
        Self {
            trades,
            price_updates,
            current_page: AppPage::Trades,
            trade_filter: TradeFilter::All,
            coin_filter: String::new(),
            trader_filter: String::new(),
            selected_tab: 0,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            scroll_offset: 0,
            tracked_coin: None,
            latest_price: None,
        }
    }

    pub fn switch_page(&mut self) {
        self.current_page = match self.current_page {
            AppPage::Trades => AppPage::PriceTracker,
            AppPage::PriceTracker => AppPage::Trades,
        };
        self.scroll_offset = 0;
    }

    pub fn start_coin_selection(&mut self) {
        self.input_mode = InputMode::CoinSelection;
        self.input_buffer = self.tracked_coin.clone().unwrap_or_default();
    }

    pub fn confirm_coin_selection(&mut self) -> Option<String> {
        if !self.input_buffer.trim().is_empty() {
            self.tracked_coin = Some(self.input_buffer.trim().to_uppercase());
            self.input_mode = InputMode::Normal;
            self.scroll_offset = 0;
            self.latest_price = None;
            return Some(self.input_buffer.trim().to_uppercase());
        }
        self.input_mode = InputMode::Normal;
        None
    }

    pub fn update_latest_price(&mut self, price_update: PriceUpdate) {
        if let Some(ref tracked) = self.tracked_coin {
            if price_update.coin_symbol == *tracked {
                self.latest_price = Some(price_update);
            }
        }
    }

    pub fn get_tracked_price_updates(&self) -> Vec<PriceUpdate> {
        if let Some(ref tracked) = self.tracked_coin {
            let updates = self.price_updates.lock().unwrap();
            updates
                .iter()
                .filter(|update| update.coin_symbol == *tracked)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn filtered_trades(&self) -> Vec<Trade> {
        let trades = self.trades.lock().unwrap();
        trades
            .iter()
            .filter(|trade| {
                let type_match = match self.trade_filter {
                    TradeFilter::All => trade.msg_type == "all-trades",
                    TradeFilter::Large => trade.msg_type == "live-trade",
                };
                
                let coin_match = self.coin_filter.is_empty() 
                    || trade.data.coin_symbol.to_lowercase().contains(&self.coin_filter.to_lowercase());
                
                let trader_match = self.trader_filter.is_empty() 
                    || trade.data.username.to_lowercase().contains(&self.trader_filter.to_lowercase());
                
                type_match && coin_match && trader_match
            })
            .cloned()
            .collect()
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        let max_items = match self.current_page {
            AppPage::Trades => self.filtered_trades().len(),
            AppPage::PriceTracker => self.get_tracked_price_updates().len(),
        };
        if self.scroll_offset < max_items.saturating_sub(1) {
            self.scroll_offset += 1;
        }
    }

    pub fn switch_trade_filter(&mut self) {
        self.trade_filter = match self.trade_filter {
            TradeFilter::All => TradeFilter::Large,
            TradeFilter::Large => TradeFilter::All,
        };
        self.scroll_offset = 0;
    }

    pub fn start_coin_filter(&mut self) {
        self.input_mode = InputMode::CoinFilter;
        self.input_buffer = self.coin_filter.clone();
    }

    pub fn start_trader_filter(&mut self) {
        self.input_mode = InputMode::TraderFilter;
        self.input_buffer = self.trader_filter.clone();
    }

    pub fn confirm_filter(&mut self) {
        match self.input_mode {
            InputMode::CoinFilter => self.coin_filter = self.input_buffer.clone(),
            InputMode::TraderFilter => self.trader_filter = self.input_buffer.clone(),
            _ => {}
        }
        self.input_mode = InputMode::Normal;
        self.scroll_offset = 0;
    }

    pub fn cancel_filter(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    pub fn add_to_input(&mut self, c: char) {
        self.input_buffer.push(c);
    }

    pub fn delete_from_input(&mut self) {
        self.input_buffer.pop();
    }
}