use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct GoldApiResponse {
    price: f64,
    prev_close_price: f64,
    chp: f64,
}

pub async fn get_gold() -> (f64, f64, f64) {
    let api_key = env::var("GOLDAPI_KEY")
        .expect("GOLDAPI_KEY not set in .env");

    let client = reqwest::Client::new();

    let res = client
        .get("https://www.goldapi.io/api/XAU/USD")
        .header("x-access-token", api_key)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to fetch gold price")
        .json::<GoldApiResponse>()
        .await
        .expect("Invalid gold JSON");

    (res.price, res.prev_close_price, res.chp)
}
