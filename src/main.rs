//TODO : manage parallel requests (tokio)
//TODO : make a simple TUI

use std::collections::HashMap;
use tokio::sync::RwLock;
use strum_macros::AsRefStr;


pub const UPDATE_SECONDS: u64 = 60;
pub const BASE_API_URL: &str = "https://api.exchangerate-api.com/v4/latest/USD"; 

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, AsRefStr)]
enum Currency{
    EUR,
    USD,
    JPY
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct RatesResponse{
    rates: HashMap<String, f64>
}

struct CurrencyValueTuple {
    price: f64,
    last_updated: std::time::Instant
}

struct CurrencyConverter{
    currencies: RwLock<HashMap<Currency, CurrencyValueTuple>>
}

impl CurrencyConverter {
    pub fn new() -> Self {
        Self { currencies: RwLock::new(HashMap::new()) }

    }
    async fn refresh_currency(&self, code: Currency) -> Result<(), Box<dyn std::error::Error>> {
        let symbol = code.as_ref();

        let url = BASE_API_URL;
        let response: RatesResponse = reqwest::get(url).await?.json().await?;

        let mut writable_currencies = self.currencies.write().await;


        if let Some(&price) = response.rates.get(symbol) {
            writable_currencies.insert(code, CurrencyValueTuple {
                price,
                last_updated: std::time::Instant::now(),
            });
        }
        Ok(())
    }
    pub async fn convert(&self, from: &Currency, to: &Currency, quantity: f64) 
        -> Option<f64> {
        if self.is_dated(from).await {
            //TODO : manage error
            let _ = self.refresh_currency(*from).await;
        }
    
        if self.is_dated(to).await {
            //TOOD : manage error
            let _ = self.refresh_currency(*to).await;
        }
        let readable_currencies = self.currencies.read().await;
        let from_rate = readable_currencies.get(from)?.price;
        let to_rate = readable_currencies.get(to)?.price;

        Some(quantity/from_rate*to_rate)
    }
    async fn is_dated(&self, code: &Currency) -> bool {
        let readable_currencies = self.currencies.read().await;
        match readable_currencies.get(code) {
            None => true, 
            Some(entry) => entry.last_updated.elapsed() > std::time::Duration::from_secs(UPDATE_SECONDS),
        }
    }
}

#[tokio::main]
async fn main() {
    let converter = CurrencyConverter::new();

    let (res1, res2, res3) = tokio::join!(
        converter.convert(&Currency::EUR, &Currency::USD, 100.0),
        converter.convert(&Currency::EUR, &Currency::USD, 100.0),
        converter.convert(&Currency::EUR, &Currency::JPY, 100.0)
    );

    println!("Conversion 1: EUR to USD : {:?}", res1);
    println!("Conversion 2: EUR to USD : {:?}", res2);
    println!("Conversion 3: EUR to JPY : {:?}", res3);
}