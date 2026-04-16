use std::collections::HashMap;


#[derive(PartialEq, Eq, Hash, Debug)]
enum Currency{
    EUR,
    USD,
    YEN
}

struct CurrencyConverter{
    currencies: HashMap<Currency, f64>
}

impl CurrencyConverter {
    pub fn new() -> Self {
        let currencies = HashMap::from([
            (Currency::EUR, 0.5),
            (Currency::USD, 1.0),
            (Currency::YEN, 2.0),
        ]);
        Self { currencies }
    }
    pub fn convert(&self, from: &Currency, to: &Currency, quantity: f64) 
        -> Option<f64> {
        let from_rate = self.currencies.get(from)?;
        let to_rate = self.currencies.get(to)?;

        Some(quantity/from_rate*to_rate)
    }
}



fn main() {
    let converter = CurrencyConverter::new();

    let quantity = 100.00;
    let from = Currency::EUR;
    let to = Currency::USD;

    match converter.convert(&from, &to, quantity) {
        Some(result) => {
            println!("{quantity} {:?} is {result:.2} {:?}", from , to);
        }
        None => println!("Error: unsupported currency."),
    }
}