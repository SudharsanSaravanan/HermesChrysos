use serde_json::Value;

pub async fn get_usdinr() -> f64 {
    let url = "https://open.er-api.com/v6/latest/USD";

    let json: Value = reqwest::get(url)
        .await
        .expect("Failed to fetch USDINR")
        .json()
        .await
        .expect("Invalid FX JSON");

    if json["result"] != "success" {
        panic!("FX API returned failure: {}", json);
    }

    json["rates"]["INR"]
        .as_f64()
        .expect("INR rate not found in FX response")
}
