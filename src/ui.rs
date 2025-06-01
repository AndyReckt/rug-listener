use crate::app::App;
use crate::models::{AppPage, InputMode, TradeFilter};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Page tabs
            Constraint::Length(3),  // Content-specific area (filters or coin selection)
            Constraint::Min(0),     // Main content
            Constraint::Length(3),  // Help
        ])
        .split(f.area());

    draw_page_tabs(f, app, chunks[0]);
    
    match app.current_page {
        AppPage::Trades => {
            draw_filters(f, app, chunks[1]);
            draw_trades(f, app, chunks[2]);
        }
        AppPage::PriceTracker => {
            draw_coin_selection(f, app, chunks[1]);
            draw_price_tracker(f, app, chunks[2]);
        }
    }
    
    draw_help(f, app, chunks[3]);
}

fn draw_page_tabs(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let page_tabs = vec!["Trade Monitor", "Price Tracker"];
    let selected_page = match app.current_page {
        AppPage::Trades => 0,
        AppPage::PriceTracker => 1,
    };
    let tabs_widget = Tabs::new(page_tabs)
        .block(Block::default().borders(Borders::ALL).title("Pages"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .select(selected_page);
    f.render_widget(tabs_widget, area);
}

fn draw_coin_selection(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let coin_text = if app.input_mode == InputMode::CoinSelection {
        &app.input_buffer
    } else {
        app.tracked_coin.as_deref().unwrap_or("No coin selected")
    };

    let coin_style = if app.input_mode == InputMode::CoinSelection {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let coin_selection = Paragraph::new(coin_text)
        .block(Block::default().borders(Borders::ALL).title("Tracked Coin (s: select)"))
        .style(coin_style);
    f.render_widget(coin_selection, area);
}

fn draw_price_tracker(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    if app.tracked_coin.is_none() {
        let help_text = Paragraph::new("Press 's' to select a coin to track")
            .block(Block::default().borders(Borders::ALL).title("Price Tracker"))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(help_text, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Current price info
            Constraint::Min(0),     // Price history
        ])
        .split(area);

    // Draw current price info
    draw_current_price(f, app, chunks[0]);
    
    // Draw price history
    draw_price_history(f, app, chunks[1]);
}

fn draw_current_price(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let coin_symbol = app.tracked_coin.as_ref().unwrap();
    
    if let Some(ref price) = app.latest_price {
        let change_color = if price.change_24h >= 0.0 {
            Color::Green
        } else {
            Color::Red
        };
        
        let change_sign = if price.change_24h >= 0.0 { "+" } else { "" };
        
        let content = vec![
            Line::from(vec![
                Span::styled(
                    format!("{} - Latest Price", coin_symbol), 
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Price: $"),
                Span::styled(
                    format!("{:.8}", price.current_price),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                ),
                Span::raw("   24h Change: "),
                Span::styled(
                    format!("{}{:.2}%", change_sign, price.change_24h),
                    Style::default().fg(change_color).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(vec![
                Span::raw("Market Cap: $"),
                Span::raw(format!("{:.2}", price.market_cap)),
                Span::raw("   Volume 24h: $"),
                Span::raw(format!("{:.2}", price.volume_24h)),
            ]),
            Line::from(vec![
                Span::raw("Pool Coin: "),
                Span::raw(format!("{:.2}", price.pool_coin_amount)),
                Span::raw("   Pool Base: "),
                Span::raw(format!("{:.2}", price.pool_base_currency_amount)),
            ]),
            Line::from(vec![
                Span::raw("Last Updated: "),
                Span::styled(
                    price.received_at.format("%H:%M:%S").to_string(),
                    Style::default().fg(Color::Cyan)
                ),
            ]),
        ];
        
        let price_info = Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title("Current Price Data"));
        f.render_widget(price_info, area);
    } else {
        let waiting_text = Paragraph::new("Waiting for price data...")
            .block(Block::default().borders(Borders::ALL).title("Current Price Data"))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(waiting_text, area);
    }
}

fn draw_price_history(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let price_updates = app.get_tracked_price_updates();
    let visible_height = area.height.saturating_sub(2) as usize;
    let start_idx = app.scroll_offset;
    let end_idx = (start_idx + visible_height).min(price_updates.len());
    
    let items: Vec<ListItem> = price_updates[start_idx..end_idx]
        .iter()
        .map(|update| {
            let change_color = if update.change_24h >= 0.0 {
                Color::Green
            } else {
                Color::Red
            };
            
            let change_sign = if update.change_24h >= 0.0 { "+" } else { "" };
            
            let content = vec![
                Line::from(vec![
                    Span::raw("Price: $"),
                    Span::styled(
                        format!("{:.8}", update.current_price),
                        Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                    ),
                    Span::raw("   Change: "),
                    Span::styled(
                        format!("{}{:.2}%", change_sign, update.change_24h),
                        Style::default().fg(change_color)
                    ),
                    Span::raw("   @ "),
                    Span::styled(
                        update.received_at.format("%H:%M:%S").to_string(),
                        Style::default().fg(Color::Cyan)
                    ),
                ]),
                Line::from(vec![
                    Span::raw("  Market Cap: $"),
                    Span::raw(format!("{:.2}", update.market_cap)),
                    Span::raw("   Volume: $"),
                    Span::raw(format!("{:.2}", update.volume_24h)),
                ]),
                Line::from(""),
            ];
            
            ListItem::new(content)
        })
        .collect();

    let price_list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!("Price History ({}) - Scroll: ↑/↓/Mouse", price_updates.len())));
    f.render_widget(price_list, area);
}

fn draw_filters(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let filter_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let coin_filter_style = if app.input_mode == InputMode::CoinFilter {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };
    
    let trader_filter_style = if app.input_mode == InputMode::TraderFilter {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let coin_filter_text = if app.input_mode == InputMode::CoinFilter {
        &app.input_buffer
    } else {
        &app.coin_filter
    };

    let trader_filter_text = if app.input_mode == InputMode::TraderFilter {
        &app.input_buffer
    } else {
        &app.trader_filter
    };

    let coin_filter = Paragraph::new(coin_filter_text.as_str())
        .block(Block::default().borders(Borders::ALL).title("Coin Filter (c)"))
        .style(coin_filter_style);
    f.render_widget(coin_filter, filter_chunks[0]);

    let trader_filter = Paragraph::new(trader_filter_text.as_str())
        .block(Block::default().borders(Borders::ALL).title("Trader Filter (t)"))
        .style(trader_filter_style);
    f.render_widget(trader_filter, filter_chunks[1]);
}

fn draw_trades(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Trade type tabs
            Constraint::Min(0),     // Trades list
        ])
        .split(area);

    // Draw trade type tabs
    let tabs = vec!["All Trades", "Large Trades"];
    let selected_tab = match app.trade_filter {
        TradeFilter::All => 0,
        TradeFilter::Large => 1,
    };
    let tabs_widget = Tabs::new(tabs)
        .block(Block::default().borders(Borders::ALL).title("Trade Type"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .select(selected_tab);
    f.render_widget(tabs_widget, chunks[0]);

    // Draw trades list
    let trades = app.filtered_trades();
    let visible_height = chunks[1].height.saturating_sub(2) as usize;
    let start_idx = app.scroll_offset;
    let end_idx = (start_idx + visible_height).min(trades.len());
    
    let items: Vec<ListItem> = trades[start_idx..end_idx]
        .iter()
        .map(|trade| {
            let trade_type_color = if trade.data.trade_type == "BUY" {
                Color::Green
            } else {
                Color::Red
            };
            
            let trade_size = if trade.msg_type == "live-trade" {
                " [LARGE]"
            } else {
                ""
            };
            
            let content = vec![
                Line::from(vec![
                    Span::styled(&trade.data.trade_type, Style::default().fg(trade_type_color).add_modifier(Modifier::BOLD)),
                    Span::raw(trade_size),
                    Span::raw(" - "),
                    Span::styled(&trade.data.username, Style::default().fg(Color::Cyan)),
                    Span::raw(" @ "),
                    Span::raw(trade.received_at.format("%H:%M:%S").to_string()),
                ]),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(&trade.data.coin_symbol, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::raw(" ("),
                    Span::raw(&trade.data.coin_name),
                    Span::raw(")"),
                ]),
                Line::from(vec![
                    Span::raw("  Amount: "),
                    Span::raw(format!("{:.2}", trade.data.amount)),
                    Span::raw(" | Value: $"),
                    Span::raw(format!("{:.2}", trade.data.total_value)),
                    Span::raw(" | Price: $"),
                    Span::raw(format!("{:.8}", trade.data.price)),
                ]),
                Line::from(""),
            ];
            
            ListItem::new(content)
        })
        .collect();

    let trades_list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!("Trades ({}/{}) - Scroll: ↑/↓/Mouse", trades.len(), app.trades.lock().unwrap().len())));
    f.render_widget(trades_list, chunks[1]);
}

fn draw_help(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let help_text = match app.input_mode {
        InputMode::Normal => match app.current_page {
            AppPage::Trades => "p/Click: Pages | Tab/Click: Filter | c/Click: Coin filter | t/Click: Trader filter | ↑/↓/Mouse: Scroll | q: Quit",
            AppPage::PriceTracker => "p/Click: Pages | s/Click: Select coin | ↑/↓/Mouse: Scroll | q: Quit",
        },
        InputMode::CoinSelection => "Enter: Confirm coin | Esc: Cancel | Backspace: Delete",
        _ => "Enter: Confirm | Esc: Cancel | Backspace: Delete",
    };
    
    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .style(Style::default().fg(Color::Gray));
    f.render_widget(help, area);
}