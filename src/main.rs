mod app;
mod models;
mod ui;
mod websocket;

use anyhow::Result;
use app::{App, MAX_PRICE_UPDATES, MAX_TRADES};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEvent, MouseEventKind, MouseButton},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use models::{AppPage, InputMode, TradeFilter};
use std::{
    collections::VecDeque,
    io,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    // Shared storage
    let trades = Arc::new(Mutex::new(VecDeque::new()));
    let price_updates = Arc::new(Mutex::new(VecDeque::new()));
    let trades_clone = trades.clone();
    let price_updates_clone = price_updates.clone();

    // Channels for WebSocket messages
    let (trade_tx, mut trade_rx) = mpsc::channel(100);
    let (price_tx, mut price_rx) = mpsc::channel(100);
    let (coin_tx, coin_rx) = mpsc::channel(10);

    // Spawn WebSocket handler
    tokio::spawn(async move {
        if let Err(e) = websocket::websocket_handler(trade_tx, price_tx, coin_rx).await {
            eprintln!("WebSocket error: {}", e);
        }
    });

    // Spawn trade receiver
    tokio::spawn(async move {
        while let Some(trade) = trade_rx.recv().await {
            let mut trades = trades_clone.lock().unwrap();
            trades.push_front(trade);
            if trades.len() > MAX_TRADES {
                trades.pop_back();
            }
        }
    });

    // Spawn price update receiver
    tokio::spawn(async move {
        while let Some(price_update) = price_rx.recv().await {
            let mut updates = price_updates_clone.lock().unwrap();
            updates.push_front(price_update);
            if updates.len() > MAX_PRICE_UPDATES {
                updates.pop_back();
            }
        }
    });

    // Create app
    let mut app = App::new(trades, price_updates);

    // Main loop
    let result = run_app(&mut terminal, &mut app, coin_tx);

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut ratatui::Terminal<B>,
    app: &mut App,
    coin_tx: mpsc::Sender<String>,
) -> Result<()> {
    loop {
        // Update latest price if we have price updates
        if let Some(tracked) = app.tracked_coin.clone() {
            let latest_update = {
                let updates = app.price_updates.lock().unwrap();
                updates.iter().find(|u| u.coin_symbol == tracked).cloned()
            };
            if let Some(latest) = latest_update {
                app.update_latest_price(latest);
            }
        }

        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match app.input_mode {
                            InputMode::Normal => {
                                if handle_normal_mode_input(app, key.code, &coin_tx)? {
                                    break;
                                }
                            }
                            InputMode::CoinFilter | InputMode::TraderFilter => {
                                handle_filter_mode_input(app, key.code);
                            }
                            InputMode::CoinSelection => {
                                handle_coin_selection_input(app, key.code, &coin_tx);
                            }
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    handle_mouse_input(app, mouse, &coin_tx);
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn handle_normal_mode_input(app: &mut App, key_code: KeyCode, _coin_tx: &mpsc::Sender<String>) -> Result<bool> {
    match key_code {
        KeyCode::Char('q') => Ok(true),
        KeyCode::Char('p') => {
            app.switch_page();
            Ok(false)
        }
        KeyCode::Tab => {
            if app.current_page == AppPage::Trades {
                app.switch_trade_filter();
            }
            Ok(false)
        }
        KeyCode::Char('c') => {
            if app.current_page == AppPage::Trades {
                app.start_coin_filter();
            }
            Ok(false)
        }
        KeyCode::Char('t') => {
            if app.current_page == AppPage::Trades {
                app.start_trader_filter();
            }
            Ok(false)
        }
        KeyCode::Char('s') => {
            if app.current_page == AppPage::PriceTracker {
                app.start_coin_selection();
            }
            Ok(false)
        }
        KeyCode::Up => {
            app.scroll_up();
            Ok(false)
        }
        KeyCode::Down => {
            app.scroll_down();
            Ok(false)
        }
        _ => Ok(false),
    }
}

fn handle_filter_mode_input(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Enter => app.confirm_filter(),
        KeyCode::Esc => app.cancel_filter(),
        KeyCode::Char(c) => app.add_to_input(c),
        KeyCode::Backspace => app.delete_from_input(),
        _ => {}
    }
}

fn handle_coin_selection_input(app: &mut App, key_code: KeyCode, coin_tx: &mpsc::Sender<String>) {
    match key_code {
        KeyCode::Enter => {
            if let Some(coin_symbol) = app.confirm_coin_selection() {
                let _ = coin_tx.try_send(coin_symbol);
            }
        }
        KeyCode::Esc => app.cancel_filter(),
        KeyCode::Char(c) => app.add_to_input(c),
        KeyCode::Backspace => app.delete_from_input(),
        _ => {}
    }
}

fn handle_mouse_input(app: &mut App, mouse: MouseEvent, coin_tx: &mpsc::Sender<String>) {
    match mouse.kind {
        MouseEventKind::ScrollUp => {
            app.scroll_up();
        }
        MouseEventKind::ScrollDown => {
            app.scroll_down();
        }
        MouseEventKind::Down(button) => {
            if button == MouseButton::Left {
                handle_click(app, mouse.column, mouse.row, coin_tx);
            }
        }
        _ => {}
    }
}

fn handle_click(app: &mut App, x: u16, y: u16, _coin_tx: &mpsc::Sender<String>) {
    // Only handle clicks in normal mode
    if app.input_mode != InputMode::Normal {
        return;
    }

    // Page tabs are at y=0-2 (including borders), full width
    if y <= 2 {
        // More precise tab detection for page tabs
        if let Ok(size) = crossterm::terminal::size() {
            let tab_width = size.0 / 2;
            // Add some margin for better click detection
            if x <= tab_width + 2 {
                // Trade Monitor tab clicked (left half)
                if app.current_page != AppPage::Trades {
                    app.switch_page();
                }
            } else {
                // Price Tracker tab clicked (right half)
                if app.current_page != AppPage::PriceTracker {
                    app.switch_page();
                }
            }
        }
        return;
    }

    // Content area starts at y=3
    match app.current_page {
        AppPage::Trades => {
            // Filter area is at y=3-5
            if y >= 3 && y <= 5 {
                if let Ok(size) = crossterm::terminal::size() {
                    let filter_width = size.0 / 2;
                    if x <= filter_width {
                        // Coin filter clicked (left half)
                        app.start_coin_filter();
                    } else {
                        // Trader filter clicked (right half)
                        app.start_trader_filter();
                    }
                }
                return;
            }
            
            // Trade type tabs are at y=6-8 (the trade tabs within the trades page)
            if y >= 6 && y <= 8 {
                if let Ok(size) = crossterm::terminal::size() {
                    // More precise detection for trade type tabs
                    let tab_width = size.0 / 2;
                    if x <= tab_width + 2 {
                        // All Trades tab clicked (left half)
                        if app.trade_filter != TradeFilter::All {
                            app.switch_trade_filter();
                        }
                    } else {
                        // Large Trades tab clicked (right half)
                        if app.trade_filter != TradeFilter::Large {
                            app.switch_trade_filter();
                        }
                    }
                }
                return;
            }
        }
        AppPage::PriceTracker => {
            // Coin selection area is at y=3-5
            if y >= 3 && y <= 5 {
                app.start_coin_selection();
                return;
            }
        }
    }
}