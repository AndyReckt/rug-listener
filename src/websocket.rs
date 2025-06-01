use crate::models::{PriceUpdate, PriceWSMessage, Trade, WSMessage};
use anyhow::Result;
use chrono::Local;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

const WS_URL: &str = "wss://ws.rugplay.com/";

pub async fn websocket_handler(
    trade_tx: mpsc::Sender<Trade>, 
    price_tx: mpsc::Sender<PriceUpdate>,
    mut coin_rx: mpsc::Receiver<String>
) -> Result<()> {
    let (ws_stream, _) = connect_async(WS_URL).await?;
    let (mut write, mut read) = ws_stream.split();

    // Subscribe to channels
    let subscribe_all = serde_json::json!({
        "type": "subscribe",
        "channel": "trades:all"
    });
    let subscribe_large = serde_json::json!({
        "type": "subscribe",
        "channel": "trades:large"
    });
    let set_coin = serde_json::json!({
        "type": "set_coin",
        "coinSymbol": "@global"
    });

    write.send(Message::Text(subscribe_all.to_string().into())).await?;
    write.send(Message::Text(subscribe_large.to_string().into())).await?;
    write.send(Message::Text(set_coin.to_string().into())).await?;

    loop {
        tokio::select! {
            // Handle coin selection updates
            coin_symbol = coin_rx.recv() => {
                match coin_symbol {
                    Some(symbol) => {
                        let set_coin_msg = serde_json::json!({
                            "type": "set_coin",
                            "coinSymbol": symbol
                        });
                        if let Err(_) = write.send(Message::Text(set_coin_msg.to_string().into())).await {
                            break;
                        }
                    }
                    None => break, // Channel closed
                }
            }
            
            // Handle incoming WebSocket messages
            msg = read.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        // Try to parse as generic JSON first to check the type
                        if let Ok(value) = serde_json::from_str::<Value>(&text) {
                            if let Some(msg_type) = value.get("type").and_then(|v| v.as_str()) {
                                match msg_type {
                                    "ping" => {
                                        // Respond to ping with pong
                                        let pong_msg = serde_json::json!({
                                            "type": "pong"
                                        });
                                        if let Err(_) = write.send(Message::Text(pong_msg.to_string().into())).await {
                                            break;
                                        }
                                    }
                                    "price_update" => {
                                        if let Ok(price_msg) = serde_json::from_str::<PriceWSMessage>(&text) {
                                            let price_update = PriceUpdate {
                                                coin_symbol: price_msg.coin_symbol,
                                                current_price: price_msg.current_price,
                                                market_cap: price_msg.market_cap,
                                                change_24h: price_msg.change_24h,
                                                volume_24h: price_msg.volume_24h,
                                                pool_coin_amount: price_msg.pool_coin_amount,
                                                pool_base_currency_amount: price_msg.pool_base_currency_amount,
                                                received_at: Local::now(),
                                            };
                                            let _ = price_tx.send(price_update).await;
                                        }
                                    }
                                    _ => {
                                        // Try to parse as trade message
                                        if let Ok(ws_msg) = serde_json::from_str::<WSMessage>(&text) {
                                            let trade = Trade {
                                                msg_type: ws_msg.msg_type,
                                                data: ws_msg.data,
                                                received_at: Local::now(),
                                            };
                                            let _ = trade_tx.send(trade).await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) => break,
                    Some(Err(_)) => break,
                    None => break,
                    _ => {}
                }
            }
        }
    }

    Ok(())
}