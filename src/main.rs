use reqwest::Error;
use serde::Deserialize;
//use std::collections::HashMap;
use std::io;

#[derive(Deserialize)]
struct Rates {
    rates: std::collections::HashMap<String, f64>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Enter the amount you want to convert: ");
    let mut in_amt = String::new();
    io::stdin().read_line(&mut in_amt).expect("Failed to read line");
    let amount: f64 = in_amt.trim().parse().expect("Please type a number!");

    println!("Enter the currency you want to convert from: ");
    let mut in_curr = String::new();
    io::stdin().read_line(&mut in_curr).expect("Failed to read line");
    let in_curr = in_curr.trim().to_uppercase();

    println!("Enter the currency you want to convert to: ");
    let mut out_curr = String::new();
    io::stdin().read_line(&mut out_curr).expect("Failed to read line");
    let out_curr = out_curr.trim().to_uppercase();

    let url = "https://api.exchangerate-api.com/v4/latest/USD";
    let response = reqwest::get(url).await?.json::<Rates>().await?;

    match response.rates.get(&out_curr) {
        Some(rate) => {
            let converted_amount = amount * rate;
            println!("{} {} is {} {}", in_amt, in_curr, converted_amount, out_curr);
        }
        None => println!("Currency code not found."),
    }

    Ok(())
}
