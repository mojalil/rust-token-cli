<div align="center">

# 🪙 Token Price CLI

A lightning-fast command-line interface tool built in Rust for retrieving real-time cryptocurrency token prices from Binance.

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Binance](https://img.shields.io/badge/Binance-FCD535?style=for-the-badge&logo=binance&logoColor=white)](https://www.binance.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Twitter](https://img.shields.io/badge/Twitter-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white)](https://x.com/motypes)


[Features](#-features) •
[Installation](#-installation) •
[Usage](#-usage) •
[Contributing](#-contributing) •
[License](#-license)

</div>

## 📋 Features

- ⚡️ **Lightning Fast**: Get real-time token prices in milliseconds
- 🔄 **Real-time Data**: Direct integration with Binance API
- 🎯 **Simple Interface**: Just one command to get any token price
- 🛡️ **Robust Error Handling**: Clear error messages and graceful failure handling
- 🔒 **Type Safe**: Built with Rust's strong type system
- 📦 **Zero Config**: Works out of the box
- 💻 **Cross Platform**: Runs on Windows, macOS, and Linux

## 🚀 Installation

### Prerequisites

- Rust (1.70.0 or higher)
- Cargo (included with Rust)

### From Source

```bash
# Clone the repository
git clone https://github.com/motypes/token-price-cli.git
cd token-price-cli

# Build and install
cargo install --path .
```

### From Cargo

```bash
cargo install token-price-cli
```

## 💻 Usage

```bash
# Get Bitcoin price in USDT
token-price-cli BTCUSDT

# Get Ethereum price in USDT
token-price-cli ETHUSDT

# Get Solana price in USDT
token-price-cli SOLUSDT
```

### Example Output
```bash
$ token-price-cli BTCUSDT
The price of BTCUSDT is $43521.32
```

## 🔧 Technical Details

- **API Endpoint**: Uses Binance's public API (`api.binance.com/api/v3/ticker/price`)
- **HTTP Client**: Implements `reqwest` with blocking client for simplicity
- **Serialization**: Efficient JSON parsing with `serde`
- **Error Handling**: Comprehensive error handling for network and parsing issues

## 📦 Dependencies

```toml
reqwest = { version = "0.11", features = ["blocking", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 🤝 Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for more information.

## 📝 License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for more information.

## 💖 Support

If you find this project helpful, please consider:
- Giving it a ⭐️ on GitHub
- Following [@motypes](https://x.com/motypes) on Twitter
- Sharing it with others

## 🙏 Acknowledgments

- [Binance API](https://binance-docs.github.io/apidocs/) for providing the price data
- The Rust community for the amazing ecosystem
- All the contributors who have helped this project grow

<div align="center">

Made with ❤️ by [@motypes](https://x.com/motypes)

</div>