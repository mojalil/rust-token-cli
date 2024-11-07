use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct TokenPrice {
    symbol: String,
    price: f64,
}
// API URL for Binance
const BINANCE_API_URL: &str = "https://api.binance.com/api/v3/ticker/price?symbol=";
// API URL for CoinGecko
const COINGECKO_API_URL: &str = "https://api.coingecko.com/api/v3/simple/price?ids=";

fn fetch_token_price(symbol: &str) -> Result<f64, Box<dyn Error>> {
    let url = format!("{}{}", BINANCE_API_URL, symbol);
    let client = Client::new();
    let response = client.get(&url).send()?.json::<serde_json::Value>()?;
    // Parse the response to get the price
    let price = response[symbol]["usd"].as_f64().ok_or("Price not found")?;

    Ok(price)
}

fn main(){
    let token = "BTCUSDT";
    match fetch_token_price(token){
        Ok(price) => println!("The price of {} is {}", token, price),
        Err(e) => println!("Error fetching price: {}", e),
    }
}