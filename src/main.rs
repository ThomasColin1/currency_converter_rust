use std::collections::HashMap;


#[derive(PartialEq, Eq, Hash, Debug)]
enum Currency{
    EUR,
    USD,
    JPY
}

struct CurrencyConverter{
    currencies: HashMap<Currency, f64>
}

impl CurrencyConverter {
    pub fn new() -> Self {
        let currencies = HashMap::from([
            (Currency::EUR, 0.5),
            (Currency::USD, 1.0),
            (Currency::JPY, 2.0),
        ]);
        Self { currencies }
    }
    pub fn update_from_web(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let url = "https://api.exchangerate-api.com/v4/latest/USD";
        let json: serde_json::Value = reqwest::blocking::get(url)?.json()?;

        if let Some(eur) = json["rates"]["EUR"].as_f64() {
            self.currencies.insert(Currency::EUR, eur);
        }
        if let Some(usd) = json["rates"]["USD"].as_f64() {
            self.currencies.insert(Currency::USD, usd);
        }
        if let Some(jpy) = json["rates"]["JPY"].as_f64() {
            self.currencies.insert(Currency::JPY, jpy);
        }

        Ok(())
    }
    pub fn convert(&self, from: &Currency, to: &Currency, quantity: f64) 
        -> Option<f64> {
        let from_rate = self.currencies.get(from)?;
        let to_rate = self.currencies.get(to)?;

        Some(quantity/from_rate*to_rate)
    }
}



fn main() {
    let mut converter = CurrencyConverter::new();

    let quantity = 100.00;
    let from = Currency::EUR;
    let to = Currency::USD;

    if let Err(e) = converter.update_from_web() {
        println!("Error while updating from API : {}", e);
    } else {
        match converter.convert(&from, &to, quantity) {
            Some(result) => {
                println!("{quantity} {:?} is {result:.2} {:?}", from , to);
            }
            None => println!("Error: unsupported currency."),
        }
    }

}