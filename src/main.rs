use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Rates {
    rates: std::collections::HashMap<String, f64>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://api.exchangerate-api.com/v4/latest/USD";
    let response = reqwest::get(url).await?.json::<Rates>().await?;

    //println!("Exchange rates: {:?}", response.rates);

    // Example conversion: USD to EUR
    let amount_usd = 100.0;
    let rate_to_eur = response.rates.get("PKR").unwrap_or(&0.0);
    let converted_amount = amount_usd * rate_to_eur;

    println!("{} USD = {} PKR", amount_usd, converted_amount);

    Ok(())
}
