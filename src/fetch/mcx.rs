use std::process::Command;
use serde_json::Value;

pub fn get_mcx_gold() -> (String, f64, f64) {
    let output = Command::new("python")
        .arg("python/groww_mcx.py")
        .output()
        .expect("Failed to run Groww MCX script");

    if !output.status.success() {
        panic!(
            "Groww MCX script failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // 🔑 Take the LAST non-empty line (JSON)
    let json_line = stdout
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .expect("No JSON output from Groww");

    let json: Value = serde_json::from_str(json_line)
        .expect("Invalid JSON from Groww MCX");

    let symbol = json["symbol"]
        .as_str()
        .expect("Missing symbol")
        .to_string();

    let ltp = json["ltp"]
        .as_f64()
        .expect("Missing ltp");

    let prev = json["prev_close"]
        .as_f64()
        .expect("Missing prev_close");

    (symbol, ltp, prev)
}
