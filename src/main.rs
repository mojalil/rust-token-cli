use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct BinancePrice {
    price: String,
}

// API URL for Binance
const BINANCE_API_URL: &str = "https://api.binance.com/api/v3/ticker/price?symbol=";

fn fetch_token_price(symbol: &str) -> Result<f64, Box<dyn Error>> {
    let url = format!("{}{}", BINANCE_API_URL, symbol);
    let client = Client::new();
    let response = client.get(&url).send()?.json::<BinancePrice>()?;
    
    // Parse the string price to f64
    let price = response.price.parse::<f64>()?;
    Ok(price)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: token-price-cli <TOKEN_SYMBOL>");
        println!("Example: token-price-cli BTCUSDT");
        return;
    }

    let token = &args[1].to_uppercase();
    match fetch_token_price(token) {
        Ok(price) => println!("The price of {} is ${:.2}", token, price),
        Err(e) => println!("Error fetching price: {}", e),
    }
}