# 🚀 Rug Trade Listener

A real-time terminal-based application for monitoring rugplay.com trades and price updates with an intuitive interactive interface.

## 📹 Demo

<video width="100%" controls>
  <source src="video/rug-listener.mp4" type="video/mp4">
  Your browser does not support the video tag.
</video>

_Watch the application in action - real-time trade monitoring and price tracking with mouse and keyboard interaction_

## ✨ Features

### 📊 **Real-Time Trade Monitoring**

-   Live trade feed with buy/sell indicators
-   Large trade highlighting for significant transactions
-   User and coin filtering capabilities
-   Scrollable trade history with timestamps

### 💰 **Price Tracking**

-   Real-time price updates for selected cryptocurrencies
-   24-hour change indicators with color coding
-   Market cap and volume information
-   Historical price data with scrollable timeline

### 🖱️ **Interactive Interface**

-   **Mouse Support**: Click on tabs, filters, and buttons
-   **Keyboard Shortcuts**: Full keyboard navigation
-   **Mouse Wheel Scrolling**: Navigate through data easily
-   **Responsive Design**: Adapts to terminal size

## 🛠️ Installation & Setup

### Prerequisites

-   **Rust** (1.70+ recommended)
-   **Cargo** package manager
-   Terminal with mouse support

### Building from Source

1. **Clone the repository**

    ```bash
    git clone https://github.com/andyreckt/rug-listener
    cd rug-trade-listener
    ```

2. **Build the application**

    ```bash
    cargo build --release
    ```

3. **Run the application**

    ```bash
    cargo run
    ```

### Quick Start (Development)

```bash
# Clone and run in one go
git clone https://github.com/andyreckt/rug-listener
cd rug-trade-listener
cargo run
```

## 🎯 Usage

### Navigation

-   **`p`** or **Click**: Switch between Trade Monitor and Price Tracker
-   **`↑/↓`** or **Mouse Wheel**: Scroll through data
-   **`q`**: Quit application

### Trade Monitor

-   **`Tab`** or **Click**: Switch between All Trades and Large Trades
-   **`c`** or **Click**: Filter trades by coin symbol
-   **`t`** or **Click**: Filter trades by trader username

### Price Tracker

-   **`s`** or **Click**: Select a coin to track
-   Real-time price updates with visual indicators
-   Historical price data with timestamps

### Mouse Interaction

-   **Click on tabs** to switch pages
-   **Click on filters** to activate them
-   **Click on coin selection** to choose tracked coins
-   **Mouse wheel** for scrolling through data

## 🏗️ Architecture

### Core Components

-   **`main.rs`**: Application entry point and event handling
-   **`app.rs`**: Application state management and business logic
-   **`ui.rs`**: Terminal user interface with ratatui
-   **`websocket.rs`**: WebSocket client for real-time data
-   **`models.rs`**: Data structures and message types

### Uses

-   **[ratatui](https://github.com/ratatui-org/ratatui)**: Terminal UI framework
-   **[tokio-tungstenite](https://github.com/snapview/tokio-tungstenite)**: Async WebSocket client
-   **[crossterm](https://github.com/crossterm-rs/crossterm)**: Cross-platform terminal manipulation
-   **[serde](https://github.com/serde-rs/serde)**: JSON serialization/deserialization

## 🤝 Contributing

Contributions are appreciated! Here's how you can help:

-   🐛 Bug Reports
-   💡 Feature Requests
-   🔧 Pull Requests

## 📞 Support

-   **Issues**: [GitHub Issues](https://github.com/yourusername/rug-trade-listener/issues)
-   **Discussions**: [GitHub Discussions](https://github.com/yourusername/rug-trade-listener/discussions)

---

**Made with ❤️, Claude Sonnet 4 and Rust** 🦀
