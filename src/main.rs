use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct TokenPrice {
    symbol: String,
    price: f64,
}

fn fetch_token_price(symbol: &str) -> Result<f64, Box<dyn Error>> {
    
}