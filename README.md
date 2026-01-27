# HermesChrysos  
**Early-Morning Gold Market Insight Engine (Educational Project)**

## ⚠︎ Important Disclaimer

This project is built **strictly for educational and learning purposes**.  
It is **NOT financial advice** and **must not be used for live trading decisions**.

## Something Most People Don’t Know About Gold Prices

Most retail investors assume that **gold prices in India start reacting only after the MCX market opens**.

That is **not true**.

In reality, **gold price direction for the day is often decided hours before Indian markets open**, based on:

- **COMEX Gold (XAU/USD)** trading overnight in global markets  
- **USD → INR currency movement**
- **Early positioning in MCX futures**

This early analysis is **routinely done by**:
- Commodity trading desks  
- Jewellery businesses  
- Treasury teams  
- Hedging & risk managers  

They don’t try to “predict exact prices”.  
Instead, they **estimate the likely direction and strength** of gold movement **before MCX opens**, to plan buying, selling, or hedging decisions.

**HermesChrysos** is a simplified, transparent implementation of this idea — built to help understand *how such predictions are structured*.

## What HermesChrysos Does

HermesChrysos generates a **India-time gold outlook** using:

1. **COMEX Gold (XAU/USD)** – global sentiment  
2. **USD → INR** – currency impact on Indian gold prices  
3. **MCX Gold Futures** – local market positioning  

It then combines these inputs using a **weighted-factor model** to produce:

- Directional bias (Bullish / Bearish / Neutral)
- Confidence level
- Expected intraday range (approximate)

## Project Architecture

```

HERMESCHRYSOS/
│
├── python/                     # Python helper scripts (Groww API)
│   ├── cache/
│   │   └── mcx-gold.json        # Cached active MCX contract data
│   ├── groww_mcx.py             # Fetch live MCX Gold futures data
│   └── update_symbol.py         # Update active MCX Gold contract (daily)
│
├── src/                         # Rust prediction engine
│   ├── fetch/
│   │   ├── gold.rs              # COMEX Gold fetcher
│   │   ├── fx.rs                # USD → INR fetcher
│   │   ├── mcx.rs               # Reads MCX data from Python cache
│   │   └── mod.rs
│   ├── config.rs                # Env & config loader
│   ├── predict.rs               # Prediction logic
│   └── main.rs                  # Final report output
│
├── .env                         # API keys
├── Cargo.toml
├── Cargo.lock
└── README.md

````

## Execution Flow (IMPORTANT)

**This order matters**

### 1️. Run Python script (once per day)

MCX Gold contracts change frequently.  
Groww requires the **exact active futures symbol**.

So you must first update the active MCX contract **before running Rust**:

```bash
python python/update_symbol.py
````

This script:

* Finds the nearest active MCX Gold futures contract
* Fetches live MCX price data
* Saves it into:

  ```
  python/cache/mcx-gold.json
  ```

---

### 2. Run Rust prediction engine

After the MCX data is cached:

```bash
cargo run
```

This will:

* Fetch COMEX Gold price
* Fetch USD → INR rate
* Read MCX Gold data from cache
* Explain the prediction logic
* Print the final 6:00 AM outlook

---

## Prediction Model (Explainable)

HermesChrysos uses a **weighted-factor model**:

```
Prediction Score =
(COMEX Gold % Change × 0.5)
+ (USDINR % Change × 0.3)
+ (MCX Futures Gap % × 0.2)
```

### Interpretation:

* COMEX dominates global direction
* USDINR translates global price into INR
* MCX futures confirm local sentiment

This structure mirrors **real-world desk-level analysis**, but in a simplified and transparent form.

## Example Output
<img width="1178" height="912" alt="image" src="https://github.com/user-attachments/assets/8517adda-1615-43b8-9813-1fbfbb728a4b" />


## Notes

* This project focuses on **directional insight**, not exact prices
* Designed for **learning, analysis, and demonstrations**
* Uses **explainable logic**, not black-box ML

## Future Improvements

* Persist previous-day USDINR & MCX close
* Backtest prediction accuracy
* Add Telegram / Email alerts
* Add volatility-based confidence scaling


## Author

Built as a learning project to understand
**how early-morning gold market behavior is estimated in practice**.

---

# Install Python dependencies (For Groww API) with:

```bash
pip install -r requirements.txt
```

