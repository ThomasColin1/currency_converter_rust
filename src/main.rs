//TODO : manage parallel requests (tokio)
//TODO : make a simple TUI

use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Currency{
    EUR,
    USD,
    JPY
}

struct CurrencyValueTuple {
    price: f64,
    last_updated: std::time::Instant
}

struct CurrencyConverter{
    currencies: HashMap<Currency, CurrencyValueTuple>
}

impl CurrencyConverter {
    pub fn new() -> Self {
        Self { currencies: HashMap::new() }

    }
    fn refresh_currency(&mut self, code: Currency) -> Result<(), Box<dyn std::error::Error>> {
        //TODO : no repetition ?
        let symbol = match code {
            Currency::EUR => "EUR",
            Currency::USD => "USD",
            Currency::JPY => "JPY",
        };

        let url = "https://api.exchangerate-api.com/v4/latest/USD";
        let json: serde_json::Value = reqwest::blocking::get(url)?.json()?;

        if let Some(price) = json["rates"][symbol].as_f64() {
            self.currencies.insert(code, CurrencyValueTuple {
                price,
                last_updated: std::time::Instant::now(),
            });
        }
        Ok(())
    }
    pub fn convert(&mut self, from: &Currency, to: &Currency, quantity: f64) 
        -> Option<f64> {
        if self.is_dated(from) {
            //TODO : manage error
            let _ = self.refresh_currency(*from);
        }
    
        if self.is_dated(to) {
            //TOOD : manage error
            let _ = self.refresh_currency(*to);
        }
        let from_rate = self.currencies.get(from)?.price;
        let to_rate = self.currencies.get(to)?.price;

        Some(quantity/from_rate*to_rate)
    }
    fn is_dated(&self, code: &Currency) -> bool {
        match self.currencies.get(code) {
            None => true, 
            Some(entry) => entry.last_updated.elapsed() > std::time::Duration::from_secs(60),
        }
    }
}


fn main() {
    let mut converter = CurrencyConverter::new();
    println!("Conversion 1 : EUR to USD : {:?}", converter.convert(&Currency::EUR, &Currency::USD, 100.0));
    println!("Conversion 2 : EUR to USD : {:?}", converter.convert(&Currency::EUR, &Currency::USD, 100.0));
    println!("Conversion 3 : EUR to JPY : {:?}", converter.convert(&Currency::EUR, &Currency::JPY, 100.0));
}