use reqwest::Error;
use serde::Deserialize;
use std::io;

#[derive(Deserialize)]
struct ApiResponse {
    // Add or modify fields to match the actual response structure
    conversion_rates: std::collections::HashMap<String, f64>,
    time_last_update_utc: String, 

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

   
    let api_key = "dcbbe4ba8c26402741310cfc"; // Use your API key here
    let url = format!("https://v6.exchangerate-api.com/v6/{}/latest/{}", api_key, in_curr);
    
    let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;


    match response.conversion_rates.get(&out_curr) {
        Some(rate) => {
            let converted_amount = amount * rate;

            println!("{} {} is {} {}",amount, in_curr, converted_amount, out_curr);

            println!("Rates last updated at: {}", response.time_last_update_utc);

        }
        None => println!("Currency code not found."),
    }

    Ok(())
}

